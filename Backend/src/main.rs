use actix_web::{App, HttpServer, web};
use sqlx::Connection;
use sqlx_postgres::{PgConnectOptions, PgPool};

mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let options = PgConnectOptions::new()
        .host("postgres_db")
        .port(5432)
        .database("postgres")
        .username("root")
        .password("password");

    let pool = PgPool::connect_with(options).await.expect("Failed to connect to DB");

    println!("Listening on 127.0.0.1:8080");
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone()))
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
