use std::error::Error;
use std::net::SocketAddrV4;
use std::sync::Arc;
use std::time::Duration;

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use zero2prod::configuration;
use zero2prod::infrastructure::SubscriptionSqlxRepository;
use zero2prod::startup;
use zero2prod::telemetry::initialise_tracing;

#[tokio::main]
async fn main() -> Result<(), impl Error> {
    initialise_tracing();

    let env: configuration::Environment = std::env::var("ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to determine environment");
    let configuration =
        configuration::get_configuration(env).expect("Failed to read configuration");

    let address = SocketAddrV4::new(
        configuration
            .application
            .host
            .parse()
            .expect("Failed to parse host"),
        configuration.application.port,
    );
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind a port");

    let repository = SubscriptionSqlxRepository::new(
        PgPoolOptions::new()
            .min_connections(5)
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to create database connection pool"),
    );

    let state = startup::Container {
        repository: Arc::new(repository),
    };

    startup::run(listener, state).await
}
