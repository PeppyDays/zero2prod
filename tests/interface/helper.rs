use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::SocketAddrV4;
use std::time::Duration;

use reqwest::Client;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::Connection;
use sqlx::Executor;
use sqlx::PgConnection;
use sqlx::Pool;
use sqlx::Postgres;
use tokio::net::TcpListener;
use uuid::Uuid;
use wiremock::matchers::method;
use wiremock::matchers::path;
use wiremock::Mock;
use wiremock::MockServer;
use wiremock::ResponseTemplate;
use zero2prod::configuration;
use zero2prod::domain;
use zero2prod::infrastructure;
use zero2prod::interface;

pub struct TestApp {
    // TODO: Need to clean up property names and how to use,
    // e.g. database_pool now might be changed to repository.pool fixture
    pub server_address: SocketAddr,
    pub database_pool: Pool<Postgres>,
    pub http_client: Client,
    pub email_server: MockServer,
}

impl TestApp {
    pub async fn new() -> Self {
        // Get configuration
        let mut configuration = configuration::get_configuration(configuration::Environment::Test)
            .expect("Failed to read configuration");
        configuration.subscriber.database.connection.database = Uuid::new_v4().to_string();

        // Create a listener
        let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
            .await
            .expect("Failed to start an app in test");
        let server_address = listener.local_addr().unwrap();

        // initialise randomise database
        TestApp::initialise_database(&configuration).await;

        // Create dependencies for injection
        let pool = PgPoolOptions::new()
            .min_connections(5)
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .connect(
                configuration
                    .subscriber
                    .database
                    .connection_string()
                    .expose_secret(),
            )
            .await
            .expect("Failed to create database connection pool");
        let subscriber_repository =
            infrastructure::subscriber::repository::SqlxSubscriberRepository::new(pool.clone());
        let subscription_token_repository =
            infrastructure::subscriber::repository::SqlxSubscriptionTokenRepository::new(
                pool.clone(),
            );
        let email_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/email"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&email_server)
            .await;
        let email_client = infrastructure::subscriber::email_client::FakeEmailClient::new(
            Client::new(),
            email_server.uri(),
            configuration.subscriber.email.client.sender,
            configuration.subscriber.email.server.token,
            configuration.subscriber.email.client.timeout,
        );
        let execute_subscriber_command =
            domain::subscriber::service::command::interface::new_execute_command(
                subscriber_repository,
                subscription_token_repository,
                email_client,
            );

        // migrate database
        sqlx::migrate!("./migrations")
            .run(&pool.clone())
            .await
            .expect("Failed to migrate the database");

        // Create a server
        let server = interface::runner::run(listener, execute_subscriber_command);
        tokio::spawn(server);

        // Create a client
        let http_client = Client::new();

        // Return service
        TestApp {
            server_address,
            database_pool: pool,
            http_client,
            email_server,
        }
    }

    async fn initialise_database(configuration: &configuration::Configuration) {
        // create a connection to postgres database
        // and create randomised database
        let original_connection_string = configuration.subscriber.database.connection_string();
        let (connection_string_without_database, _) = original_connection_string
            .expose_secret()
            .rsplit_once("/")
            .expect("Failed to parse database connection string");

        let mut connection = PgConnection::connect(connection_string_without_database)
            .await
            .expect("Failed to connect to Postgres");

        connection
            .execute(
                format!(
                    r#"CREATE DATABASE "{}";"#,
                    configuration.subscriber.database.connection.database
                )
                .as_str(),
            )
            .await
            .expect("Failed to create database.");
    }

    pub fn get_server_request_url(&self, path: &str) -> String {
        format!(
            "http://{}/{}",
            self.server_address,
            path.trim_start_matches("/")
        )
    }
}
