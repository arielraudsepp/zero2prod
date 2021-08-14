//! main.rs
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration");
    tracing::info!("Trying to connect to database at {}:{}", configuration.database.host, configuration.database.port);
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(300))
        .connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
