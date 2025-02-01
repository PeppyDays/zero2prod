use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::SocketAddrV4;

use reqwest::header;
use reqwest::Response;
use reqwest::StatusCode;
use secrecy::ExposeSecret;
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
use zero2prod::aggregates::subscriber;
use zero2prod::aggregates::subscriber::domain::service::CommandExecutor as SubscriberCommandExecutor;
use zero2prod::assembly;
use zero2prod::assembly::get_database_connection_string;
use zero2prod::configuration;
use zero2prod::interface;

pub struct System {
    pub requestor: SystemRequestor,
    pub dependencies: SystemDependencies,
}

impl System {
    pub async fn new() -> Self {
        // Read configuration
        let mut configuration =
            configuration::get_configuration(configuration::Environment::Test).unwrap();

        // Set randomized database for testing
        configuration.subscriber.database.connection.database = Uuid::now_v7().into();

        // Initialise randomised database
        let original_connection_string =
            get_database_connection_string(&configuration.subscriber.database);
        let (connection_string_without_database, _) = original_connection_string
            .expose_secret()
            .rsplit_once("/")
            .unwrap();
        let mut connection = PgConnection::connect(connection_string_without_database)
            .await
            .unwrap();
        connection
            .execute(
                format!(
                    r#"CREATE DATABASE "{}";"#,
                    configuration.subscriber.database.connection.database
                )
                .as_str(),
            )
            .await
            .unwrap();

        // Create subscriber database dependency
        let subscriber_database_pool =
            assembly::get_database_pool(&configuration.subscriber.database).await;
        let subscriber_repository =
            assembly::assemble_subscriber_repository(subscriber_database_pool.clone());
        let subscription_token_repository =
            assembly::assemble_subscription_token_repository(subscriber_database_pool.clone());
        sqlx::migrate!("./migrations")
            .run(&subscriber_database_pool)
            .await
            .unwrap();

        // Create subscriber email dependency
        let subscription_email_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/email"))
            .respond_with(ResponseTemplate::new(StatusCode::OK))
            .mount(&subscription_email_server)
            .await;
        configuration.subscriber.email.server.url = subscription_email_server.uri();
        let subscription_email_client =
            assembly::assemble_subscription_email_client(&configuration.subscriber.email);

        // Create dependencies
        let dependencies = SystemDependencies {
            subscriber_database_pool,
            subscription_email_server,
        };

        // Assemble subscriber aggregate's command executor
        let subscriber_command_executor = subscriber::domain::service::new_command_executor(
            subscriber_repository,
            subscription_token_repository,
            subscription_email_client,
        );

        // Set up listener and client
        let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
            .await
            .unwrap();
        let requestor = SystemRequestor {
            url: listener.local_addr().unwrap(),
            client: reqwest::Client::new(),
        };

        // Run a server
        tokio::spawn(interface::runner::run(
            listener,
            subscriber_command_executor,
        ));

        // Return test system
        System {
            requestor,
            dependencies,
        }
    }
}

pub struct SystemRequestor {
    pub url: SocketAddr,
    pub client: reqwest::Client,
}

impl SystemRequestor {
    pub async fn get_healthz(&self) -> Response {
        self.client.get(self.url("/healthz")).send().await.unwrap()
    }

    pub async fn post_subscriptions(
        &self,
        name: Option<String>,
        email: Option<String>,
    ) -> Response {
        let mut body = String::new();
        if let Some(name) = name {
            body.push_str(format!("&name={}", &urlencoding::encode(&name)).as_str());
        };
        if let Some(email) = email {
            body.push_str(format!("&email={}", &urlencoding::encode(&email)).as_str());
        };
        body = body.trim_start_matches("&").to_string();

        self.client
            .post(self.url("/subscriptions"))
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .unwrap()
    }

    pub async fn get_subscriptions_confirm(&self, token: Option<String>) -> Response {
        let mut request_builder = self.client.get(self.url("/subscriptions/confirm"));
        if let Some(token) = token {
            request_builder = request_builder.query(&[("token", token)])
        }

        request_builder.send().await.unwrap()
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}/{}", self.url, path.trim_start_matches("/"))
    }
}

pub struct SystemDependencies {
    pub subscriber_database_pool: Pool<Postgres>,
    pub subscription_email_server: MockServer,
}

impl SystemDependencies {
    pub async fn reset_subscription_email_server(
        &self,
        response_template: Option<ResponseTemplate>,
    ) {
        self.subscription_email_server.reset().await;

        let response_template = response_template.unwrap_or(ResponseTemplate::new(StatusCode::OK));
        Mock::given(method("POST"))
            .and(path("/email"))
            .respond_with(response_template)
            .mount(&self.subscription_email_server)
            .await;
    }
}

#[rstest::fixture]
pub async fn system() -> System {
    System::new().await
}

pub struct SystemSurface {
    pub requestor: SystemRequestor,
}

impl SystemSurface {
    pub async fn new(subscriber_command_executor: impl SubscriberCommandExecutor) -> Self {
        let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
            .await
            .unwrap();
        let requestor = SystemRequestor {
            url: listener.local_addr().unwrap(),
            client: reqwest::Client::new(),
        };

        tokio::spawn(interface::runner::run(
            listener,
            subscriber_command_executor,
        ));

        SystemSurface { requestor }
    }
}
