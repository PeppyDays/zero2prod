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
use zero2prod::configuration;
use zero2prod::subscription::domain;
use zero2prod::subscription::infrastructure;
use zero2prod::subscription::interface;

pub struct TestApp {
    pub server_address: SocketAddr,
    pub database_pool: Pool<Postgres>,
    pub http_client: Client,
}

impl TestApp {
    pub async fn new() -> Self {
        // Get configuration
        let mut configuration = configuration::get_configuration(configuration::Environment::Test)
            .expect("Failed to read configuration");
        configuration.database.database = Uuid::new_v4().to_string();

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
            .connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to create database connection pool");
        let repository = infrastructure::repository::SqlxRepository::new(pool.clone());
        let command_executor = domain::service::CommandExecutor::new(repository);

        // migrate database
        sqlx::migrate!("./migrations")
            .run(&pool.clone())
            .await
            .expect("Failed to migrate the database");

        // Create a server
        let server = interface::runner::run(listener, command_executor);
        tokio::spawn(server);

        // Create a client
        let http_client = Client::new();

        // Return service
        TestApp {
            server_address,
            database_pool: pool,
            http_client,
        }
    }

    async fn initialise_database(configuration: &configuration::Configuration) {
        // create a connection to postgres database
        // and create randomised database
        let original_connection_string = configuration.database.connection_string();
        let (connection_string_without_database, _) = original_connection_string
            .expose_secret()
            .rsplit_once("/")
            .expect("Failed to parse database connection string");

        let mut connection = PgConnection::connect(connection_string_without_database)
            .await
            .expect("Failed to connect to Postgres");

        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, configuration.database.database).as_str())
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
