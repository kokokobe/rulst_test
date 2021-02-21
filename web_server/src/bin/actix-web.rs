use actix_web::{get, web, App, HttpServer, Responder, guard, HttpResponse, Result, http::StatusCode};
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(home)
        // default
        .default_service(
            // 404 for GET request
            web::resource("")
                .route(web::get().to(p404))
                // all requests that are not `GET`
                .route(
                    web::route()
                        .guard(guard::Not(guard::Get()))
                        .to(HttpResponse::MethodNotAllowed),
                ),
        ))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[get("/")]
async fn home() -> impl Responder {
    format!("Hello! actix-home")
}

/// 404 handler
async fn p404() -> Result<fs::NamedFile> {
    let env = env!("CARGO_MANIFEST_DIR");
    let file_name = "static/404.html";
    let path = env.to_owned() + "/" + file_name;
    println!("env is {}", env);
    println!("file path is:{}", path);
    let result = fs::NamedFile::open(path);
    let named_file = result.unwrap();
    Ok(named_file.set_status_code(StatusCode::NOT_FOUND))
}