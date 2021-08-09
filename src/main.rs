//! main.rs
use std::net::TcpListener;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("Attempting to listen on http://127.0.0.1:{}",port);
    let server = run(listener).expect("Failed to run server");
    server.await?;
    Ok(())
}
