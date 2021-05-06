use std::borrow::Borrow;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use actix::dev::MessageResponse;
use actix::prelude::*;
use actix_files;
use actix_web::{App, Error, get, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web_actors::ws;
use actix_web_actors::ws::ProtocolError;
use log::info;
use log4rs;

use web_server::server::*;
use web_server::server;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("starting actix web server");
    // App state
    // We are keeping a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));
    // Start chat server actor
    let server = ChatServer::new(app_state.clone()).start();
    // Create Http server with websocket support
    HttpServer::new(move || {
        App::new().data(app_state.clone())
            .data(server.clone())
            // redirect to websocket.html
            .service(web::resource("/").route(web::get().to(|| {
                HttpResponse::Found().header("LOCATION", "/static/web_socket.html")
                    .finish()
            })))
            .route("/count", web::get().to(get_count))
            .service(chat_route)
            .service(actix_files::Files::new("/static/", "static/"))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

pub async fn get_count(count: web::Data<Arc<AtomicUsize>>) -> impl Responder {
    let current_count = count.fetch_add(1, Ordering::SeqCst);
    format!("Visitors: {}", current_count)
}

/// Entry point for our websocket route
#[get("/ws/")]
pub async fn chat_route(req: HttpRequest, stream: web::Payload, srv: web::Data<Addr<ChatServer>>)
                        -> Result<HttpResponse, Error> {
    let ws_chat_session = WsChatSession {
        id: 0,
        hb: Instant::now(),
        room: "Main".to_string(),
        name: None,
        addr: srv.get_ref().clone(),
        remote_ip: req.connection_info().remote_addr().expect("error get remote ip").to_string(),
    };
    ws::start(ws_chat_session, &req, stream)
}

struct WsChatSession {
    /// unique session id
    id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
   /// otherwise we drop connection.
    hb: Instant,
    ///joined room
    room: String,
    ///peer name
    name: Option<String>,
    ///chat server
    addr: Addr<ChatServer>,
    remote_ip: String,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);
        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr.send(Connect { addr: addr.recipient(), remote_ip: self.remote_ip.clone() }).into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            }).wait(ctx);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        // notify chat server
        let remote_ip = self.remote_ip.clone();
        let id = self.id.clone();
        self.addr.do_send(Disconnect { id, remote_ip });
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<server::Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(item) => item,
        };
        info!("WEBSOCKET MESSAGE:{:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();
                // we check for /sss type of messages
                if m.starts_with('/') {
                    let v: Vec<&str> = m.splitn(2, ' ').collect();
                    match v[0] {
                        "/list" => {
                            // Send ListRooms message to chat server and wait for response
                            info!("List rooms");
                            self.addr.send(server::ListRooms).into_actor(self)
                                .then(|res, _, ctx| {
                                    match res {
                                        Ok(rooms) => {
                                            for room in rooms {
                                                ctx.text(room);
                                            }
                                        }
                                        _ => info!("Something is wrong")
                                    }
                                    fut::ready(())
                                }).wait(ctx)
                            // .wait(ctx) pauses all events in context,
                            // so actor wont receive any new messages until it get list
                            // of rooms back
                        }
                        "/join" => {
                            if v.len() == 2 {
                                self.room = v[1].to_owned();
                                self.addr.do_send(server::Join {
                                    id: self.id,
                                    name: self.room.clone(),
                                });
                                ctx.text("joined")
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        }
                        "/name" => {
                            if v.len() == 2 {
                                self.name = Some(v[1].to_owned());
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }
                        _ => {
                            ctx.text(format!("!!! unknown command:{:?}", m))
                        }
                    }
                } else {
                    let msg = if let Some(name) = &self.name {
                        format!("{}:{}", name, m)
                    } else {
                        m.to_owned()
                    };
                    //send message to chat server
                    self.addr.do_send(server::ClientMessage {
                        id: self.id,
                        msg,
                        room: self.room.clone(),
                    })
                }
            }
            ws::Message::Binary(_) => info!("Unexpected binary!"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl WsChatSession {
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                info!("Websocket Client heartbeat failed, disconnecting!");
                // notify chat server
                act.addr.do_send(Disconnect { id: act.id, remote_ip: act.remote_ip.clone() });
                //stop actor
                ctx.stop();
                // don't try to send a ping
                return;
            }
            ctx.ping(b"")
        });
    }
}