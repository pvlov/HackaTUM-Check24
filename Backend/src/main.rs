mod server;
use actix_web::{web, App, HttpServer};
use sqlx::{Connection, query};
use sqlx_postgres::{PgConnectOptions, PgConnection};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let options = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .database("postgres")
        .username("postgres")
        .password("password");

    let pool = PgConnection::connect_with(&options).await;
    
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
