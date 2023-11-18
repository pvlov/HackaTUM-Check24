mod server;
use actix_web::{get, web, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on 127.0.0.1:8080");
    HttpServer::new(|| App::new().service(server::index).service(server::hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
