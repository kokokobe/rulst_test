use actix_web::{
    get, post, web, App, HttpServer, Responder, guard, HttpResponse, Result,
    http::StatusCode, middleware,
};
use serde_json::json;
use actix_files as fs;
use log::{info};
use log4rs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::env;
    let exe_path = env::current_exe()?;
    let exe_path_dir = exe_path.parent().expect("Executable must be in some directory");
    println!("The execute path dir is:{:?}", exe_path_dir);
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("starting actix web server");

    HttpServer::new(||
        App::new()
            //enable request response logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(index)
            .service(home)
            // default
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    .route(web::post().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed)
                            .guard(guard::Not(guard::Post()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            ))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id);
    let json = json!({"hello":"world"});
    HttpResponse::Ok().json(json)
}

#[get("/")]
async fn home() -> impl Responder {
    format!("Hello! actix-home")
}

/// 404 handler
async fn p404() -> Result<fs::NamedFile> {
    let file_name = "static/404.html";
    let result = fs::NamedFile::open(file_name);
    let named_file = result.unwrap();
    Ok(named_file.set_status_code(StatusCode::NOT_FOUND))
}