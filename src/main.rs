use std::error::Error;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use zero2prod::configuration;
use zero2prod::startup;
use zero2prod::telemetry::initialise_tracing;

#[tokio::main]
async fn main() -> Result<(), impl Error> {
    initialise_tracing();

    let configuration = configuration::get_configuration("configuration.yaml")
        .expect("Failed to read configuration");

    let address = SocketAddrV4::new(Ipv4Addr::LOCALHOST, configuration.application.port);
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind a port");

    let database_pool = PgPoolOptions::new()
        .min_connections(5)
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(configuration.database.connection_string().as_str())
        .await
        .expect("Failed to create database connection pool");

    let state = startup::AppState { database_pool };

    startup::run(listener, state).await
}
