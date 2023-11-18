mod server;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(server::index)
            .service(server::get_craftsmen_data)
            .service(
                web::resource("/craftman/{craftman_id}")
                    .route(web::patch().to(server::update_craftsman)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
