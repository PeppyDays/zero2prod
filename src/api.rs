use std::error::Error;
use std::net::SocketAddrV4;
use std::time::Duration;

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use wiremock::matchers::method;
use wiremock::matchers::path;
use wiremock::Mock;
use wiremock::MockServer;
use wiremock::ResponseTemplate;
use zero2prod::configuration;
use zero2prod::subscription;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> Result<(), impl Error> {
    telemetry::initialise_tracing();

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

    let subscriber_repository =
        subscription::infrastructure::subscriber::repository::SqlxRepository::new(
            PgPoolOptions::new()
                .min_connections(5)
                .max_connections(5)
                .acquire_timeout(Duration::from_secs(5))
                .connect(configuration.database.connection_string().expose_secret())
                .await
                .expect("Failed to create database connection pool"),
        );
    let subscriber_email_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/email"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&subscriber_email_server)
        .await;
    let subscriber_email_client =
        subscription::infrastructure::subscriber::email_client::FakeEmailClient::new(
            reqwest::Client::new(),
            subscriber_email_server.uri(),
            configuration.email_client.sender,
            configuration.email_client.token,
            configuration.email_client.timeout,
        );
    let execute_subscriber_command =
        subscription::domain::subscriber::service::command::interface::new_execute_command(
            subscriber_repository,
            subscriber_email_client,
        );

    subscription::interface::runner::run(listener, execute_subscriber_command).await
}
