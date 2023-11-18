use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Test"
}

#[get("/{post_code}")]
async fn hello(post_code: web::Path<String>) -> impl Responder {
    format!("Hello {}!\n", &post_code)
}
