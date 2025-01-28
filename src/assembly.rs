use std::net::SocketAddrV4;

use reqwest::StatusCode;
use secrecy::ExposeSecret;
use secrecy::SecretString;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use tokio::net::TcpListener;
use wiremock::matchers::method;
use wiremock::matchers::path;
use wiremock::Mock;
use wiremock::MockServer;
use wiremock::ResponseTemplate;

use crate::aggregates::subscriber::domain::infrastructure::EmailClient;
use crate::aggregates::subscriber::domain::infrastructure::SubscriberRepository;
use crate::aggregates::subscriber::domain::infrastructure::SubscriptionTokenRepository;
use crate::aggregates::subscriber::infrastructure::email_client::FakeEmailClient;
use crate::aggregates::subscriber::infrastructure::repository::SqlxSubscriberRepository;
use crate::aggregates::subscriber::infrastructure::repository::SqlxSubscriptionTokenRepository;
use crate::configuration::ApplicationConfiguration;
use crate::configuration::DatabaseConfiguration;
use crate::configuration::EmailConfiguration;

pub async fn get_application_listener(c: &ApplicationConfiguration) -> TcpListener {
    TcpListener::bind(SocketAddrV4::new(
        c.host.parse().expect("Failed to parse host as IP"),
        c.port,
    ))
    .await
    .expect("Failed to bind a port")
}

pub fn get_database_connection_string(c: &DatabaseConfiguration) -> SecretString {
    SecretString::from(format!(
        "postgres://{}:{}@{}:{}/{}",
        c.connection.username,
        c.connection.password.expose_secret(),
        c.connection.host,
        c.connection.port,
        c.connection.database,
    ))
}

pub async fn get_database_pool(c: &DatabaseConfiguration) -> Pool<Postgres> {
    PgPoolOptions::new()
        .min_connections(c.pool.min_connections)
        .max_connections(c.pool.max_connections)
        .acquire_timeout(c.pool.acquire_timeout)
        .connect(get_database_connection_string(c).expose_secret())
        .await
        .expect("Failed to create database connection pool")
}

pub fn assemble_subscriber_repository(pool: Pool<Postgres>) -> impl SubscriberRepository {
    SqlxSubscriberRepository::new(pool)
}

pub fn assemble_subscription_token_repository(
    pool: Pool<Postgres>,
) -> impl SubscriptionTokenRepository {
    SqlxSubscriptionTokenRepository::new(pool)
}

pub async fn assemble_subscription_email_server(
    c: &mut EmailConfiguration,
) -> wiremock::MockServer {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/email"))
        .respond_with(ResponseTemplate::new(StatusCode::OK))
        .mount(&server)
        .await;
    c.server.url = server.uri();
    server
}

pub fn assemble_subscription_email_client(c: &EmailConfiguration) -> impl EmailClient {
    FakeEmailClient::new(
        reqwest::Client::new(),
        c.server.url.clone(),
        c.client.sender.clone(),
        c.server.token.clone(),
        c.client.timeout,
    )
}
