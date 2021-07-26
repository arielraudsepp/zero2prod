//! lib.rs
use actix_web::{App, HttpServer, HttpResponse, Responder, web};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
