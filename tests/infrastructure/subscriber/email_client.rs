use std::sync::Arc;

use reqwest::StatusCode;
use tokio::sync::RwLock;
use wiremock::matchers::method;
use wiremock::matchers::path;
use wiremock::Mock;
use wiremock::MockServer;
use wiremock::ResponseTemplate;
use zero2prod::configuration::get_configuration;
use zero2prod::configuration::Environment;
use zero2prod::domain::subscriber::exception::Error;
use zero2prod::domain::subscriber::infrastructure::EmailClient;
use zero2prod::domain::subscriber::model::Subscriber;
use zero2prod::infrastructure::subscriber::email_client::FakeEmailClient;

#[derive(Clone)]
pub struct EmailClientDouble {
    recipient: Arc<RwLock<Option<Subscriber>>>,
    subject: Arc<RwLock<Option<String>>>,
    content: Arc<RwLock<Option<String>>>,
}

impl EmailClientDouble {
    pub fn new() -> Self {
        EmailClientDouble {
            recipient: Arc::new(RwLock::new(None)),
            subject: Arc::new(RwLock::new(None)),
            content: Arc::new(RwLock::new(None)),
        }
    }
}

impl Default for EmailClientDouble {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl EmailClient for EmailClientDouble {
    async fn send(
        &self,
        recipient: &Subscriber,
        subject: &str,
        content: &str,
    ) -> Result<(), Error> {
        *self.recipient.write().await = Some(recipient.clone());
        *self.subject.write().await = Some(subject.into());
        *self.content.write().await = Some(content.into());
        Ok(())
    }
}

#[rstest::fixture]
pub async fn email_client_double() -> EmailClientDouble {
    EmailClientDouble::new()
}

#[rstest::fixture]
pub async fn email_server_and_client(
    #[future(awt)]
    #[from(email_server)]
    server: MockServer,
) -> (MockServer, FakeEmailClient) {
    let configuration = get_configuration(Environment::Test).unwrap();

    let client = FakeEmailClient::new(
        reqwest::Client::new(),
        server.uri(),
        configuration.subscriber.email.client.sender,
        configuration.subscriber.email.server.token,
        configuration.subscriber.email.client.timeout,
    );

    (server, client)
}

#[rstest::fixture]
pub async fn faulty_email_server_and_client(
    #[future(awt)]
    #[from(email_server)]
    #[with(StatusCode::INTERNAL_SERVER_ERROR)]
    server: MockServer,
) -> (MockServer, FakeEmailClient) {
    let configuration = get_configuration(Environment::Test).unwrap();

    let client = FakeEmailClient::new(
        reqwest::Client::new(),
        server.uri(),
        configuration.subscriber.email.client.sender,
        configuration.subscriber.email.server.token,
        configuration.subscriber.email.client.timeout,
    );

    (server, client)
}

#[rstest::fixture]
pub async fn email_server(
    #[default(StatusCode::OK)] response_status_code: StatusCode,
) -> MockServer {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/email"))
        .respond_with(ResponseTemplate::new(response_status_code))
        .mount(&server)
        .await;

    server
}
