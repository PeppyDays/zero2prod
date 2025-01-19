use claims::assert_err;
use claims::assert_matches;
use claims::assert_ok;
use fake::Fake;
use wiremock::MockServer;
use zero2prod::subscription::domain::subscriber::service::command::executors::subscribe::Command as SubscribeCommand;
use zero2prod::subscription::domain::subscriber::service::command::interface::new_execute_command;
use zero2prod::subscription::domain::subscriber::service::command::interface::Command;
use zero2prod::subscription::exception::Error;
use zero2prod::subscription::infrastructure::subscriber::email_client::FakeEmailClient;
use zero2prod::subscription::infrastructure::subscriber::repository::SqlxRepository;

use crate::subscription::domain::subscriber::command::email;
use crate::subscription::domain::subscriber::command::name;
use crate::subscription::domain::subscriber::command::subscribe_command as command;
use crate::subscription::infrastructure::subscriber::email_client::email_client_double;
use crate::subscription::infrastructure::subscriber::email_client::email_server_and_client;
use crate::subscription::infrastructure::subscriber::email_client::faulty_email_server_and_client;
use crate::subscription::infrastructure::subscriber::email_client::EmailClientDouble;
use crate::subscription::infrastructure::subscriber::repository::repository;

#[rstest::rstest]
#[tokio::test]
async fn sut_stores_subscribers_correctly(
    #[future(awt)] repository: SqlxRepository,
    #[future(awt)]
    #[from(email_client_double)]
    email_client: EmailClientDouble,
    command: Command,
) {
    // Arrange
    let sut = new_execute_command(repository, email_client);

    // Act
    let actual = sut(command).await;

    // Assert
    assert_ok!(actual);
}

#[rstest::rstest]
#[tokio::test]
async fn sut_raises_invalid_attributes_error_if_name_is_longer_than_256(
    #[future(awt)] repository: SqlxRepository,
    #[future(awt)]
    #[from(email_client_double)]
    email_client: EmailClientDouble,
    email: String,
) {
    // Arrange
    let sut = new_execute_command(repository, email_client);
    let name = (0..(256..1024).fake::<u32>())
        .map(|_| "X")
        .collect::<String>();
    let command = create_subscribe_command(name, email);

    // Act
    let actual = sut(command).await;

    // Assert
    assert_err!(&actual);
    assert_matches!(&actual.unwrap_err(), Error::InvalidAttributes);
}

#[rstest::rstest]
#[tokio::test]
async fn sut_sends_sending_email_request_with_authorization_token_to_email_server_correctly(
    #[future(awt)] repository: SqlxRepository,
    #[future(awt)] email_server_and_client: (MockServer, FakeEmailClient),
    name: String,
    email: String,
) {
    // Arrange
    let (email_server, email_client) = email_server_and_client;
    let sut = new_execute_command(repository, email_client.clone());
    let command = create_subscribe_command(name, email.clone());

    // Act
    let _ = sut(command).await;

    // Assert
    let request = extract_first_received_request(email_server).await;
    let actual = request
        .headers
        .get("X-Postmark-Server-Token")
        .unwrap()
        .to_str()
        .unwrap();
    let expected = "SECRET_TOKEN";
    assert_eq!(actual, expected);
}

#[rstest::rstest]
#[tokio::test]
async fn sut_sends_sending_email_request_body_to_email_server_correctly(
    #[future(awt)] repository: SqlxRepository,
    #[future(awt)] email_server_and_client: (MockServer, FakeEmailClient),
    name: String,
    email: String,
) {
    // Arrange
    let (email_server, email_client) = email_server_and_client;
    let sut = new_execute_command(repository, email_client.clone());
    let command = create_subscribe_command(name, email.clone());

    // Act
    let _ = sut(command).await;

    // Assert
    let request = extract_first_received_request(email_server).await;
    let actual: serde_json::Value = request.body_json().unwrap();
    let expected = serde_json::json!({
        "From": "test@gmail.com",
        "To": email,
        "Subject": "hello!",
        "Content": "click this link: ..",
    });
    assert_eq!(actual, expected);
}

#[rstest::rstest]
#[tokio::test]
async fn sut_raises_failed_email_operation_error_if_email_server_responds_with_internal_server_error(
    #[future(awt)] repository: SqlxRepository,
    #[future(awt)] faulty_email_server_and_client: (MockServer, FakeEmailClient),
    name: String,
    email: String,
) {
    // Arrange
    let (_, email_client) = faulty_email_server_and_client;
    let sut = new_execute_command(repository, email_client.clone());
    let command = create_subscribe_command(name, email.clone());

    // Act
    let actual = sut(command).await.unwrap_err();

    // Assert
    assert_matches!(actual, Error::FailedEmailOperation);
}

fn create_subscribe_command(name: String, email: String) -> Command {
    Command::Subscribe(SubscribeCommand::new(name, email))
}

async fn extract_first_received_request(email_server: MockServer) -> wiremock::Request {
    email_server.received_requests().await.unwrap()[0].clone()
}
