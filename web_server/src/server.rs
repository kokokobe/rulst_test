use std::collections::{HashMap, HashSet};
use actix::prelude::*;
use rand::rngs::ThreadRng;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use actix::dev::MessageResponse;
use rand::Rng;
use uuid::Uuid;
use rand::seq::index::IndexVec::USize;
use std::convert::TryFrom;

/// `ChatServer` manages chat rooms and responsible for coordinating chat
/// session. implementation is super primitive
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
    visitor_count: Arc<AtomicUsize>,
}

impl ChatServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> ChatServer {
        let mut rooms = HashMap::new();
        rooms.insert("Main".to_owned(), HashSet::new());
        ChatServer {
            sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
            visitor_count,
        }
    }
    /// Send message to all users in the room
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id == skip_id {
                    continue;
                }
                if let Some(addr) = self.sessions.get(id) {
                    addr.do_send(Message(message.to_owned()));
                }
            }
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, ctx: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");
        self.send_message(&"Main", "Someone joined", 0);
        // register session with random id
        let id = self.rng.gen();
        self.sessions.insert(id, msg.addr);
        // auto join session to Main room
        self.rooms.entry("Main".to_owned()).or_insert_with(HashSet::new).insert(id);
        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_message("Main", &format!("Total visitors {}", count), 0);
        //send id back
        id
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Context<Self>) -> Self::Result {
        println!("Someone disconnected");
        let mut rooms: Vec<String> = Vec::new();
        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all rooms
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Context<Self>) -> Self::Result {
        self.send_message(&msg.room, msg.msg.as_str(), msg.id);
    }
}

/// Handle for `ListRooms` message.
impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, msg: ListRooms, ctx: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();
        for key in self.rooms.keys() {
            rooms.push(key.to_owned())
        }
        MessageResult(rooms)
    }
}

impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Join, ctx: &mut Context<Self>) -> Self::Result {
        let Join { id, name } = msg;
        //modify rooms
        let mut rooms = Vec::new();
        for (name, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(name.to_owned());
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
        self.rooms.entry(name.clone()).or_insert_with(HashSet::new).insert(id);
    }
}

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

///Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    //user session id
    pub id: usize
}

///New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>
}

/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub room: String,
}

pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

/// Join room, if room does not exists create new one.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    /// Client session id
    pub id: usize,
    /// Room name
    pub name: String,
}
