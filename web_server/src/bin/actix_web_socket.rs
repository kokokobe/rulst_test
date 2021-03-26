use log::{info};
use log4rs;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use actix_web::{HttpServer, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("starting actix web server");
    // App state
    // We are keeping a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));
    // Create Http server with websocket support
    HttpServer::new(move || {
        App::new().data(app_state.clone())
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}