//!src/routes/health_check.rs
use actix_web::{App, HttpServer, HttpResponse, web};
use actix_web::dev::Server;
use std::net::TcpListener;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
