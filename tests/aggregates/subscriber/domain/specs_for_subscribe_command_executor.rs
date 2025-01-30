use std::collections::HashSet;

use fake::Fake;
use zero2prod::aggregates::subscriber::domain::error::Error;
use zero2prod::aggregates::subscriber::domain::model::Email;
use zero2prod::aggregates::subscriber::domain::model::Status;
use zero2prod::aggregates::subscriber::domain::service::new_command_executor;
use zero2prod::aggregates::subscriber::domain::service::Command;
use zero2prod::aggregates::subscriber::domain::service::SubscribeCommand;
use zero2prod::aggregates::subscriber::infrastructure::email_client::FakeEmailClient;
use zero2prod::aggregates::subscriber::infrastructure::repository::SqlxSubscriberRepository;
use zero2prod::aggregates::subscriber::infrastructure::repository::SqlxSubscriptionTokenRepository;

use crate::aggregates::subscriber::domain::command::subscribe_command as command;
use crate::aggregates::subscriber::domain::command::subscribe_commands as commands;
use crate::aggregates::subscriber::domain::model::email;
use crate::aggregates::subscriber::infrastructure::email_client::email_client_double;
use crate::aggregates::subscriber::infrastructure::email_client::email_server_and_client;
use crate::aggregates::subscriber::infrastructure::email_client::extract_first_received_request;
use crate::aggregates::subscriber::infrastructure::email_client::faulty_email_server_and_client;
use crate::aggregates::subscriber::infrastructure::email_client::EmailClientDouble;
use crate::aggregates::subscriber::infrastructure::repository::find_subscriber_by_email;
use crate::aggregates::subscriber::infrastructure::repository::find_subscription_token_by_subscriber_id;
use crate::aggregates::subscriber::infrastructure::repository::subscriber_repository;
use crate::aggregates::subscriber::infrastructure::repository::subscription_token_repository;

#[rstest::rstest]
#[tokio::test]
async fn sut_stores_new_subscribers_correctly(
    #[future(awt)] subscriber_repository: SqlxSubscriberRepository,
    #[future(awt)] subscription_token_repository: SqlxSubscriptionTokenRepository,
    #[future(awt)]
    #[from(email_client_double)]
    dummy: EmailClientDouble,
    command: Command,
) {
    // Arrange
    let sut = new_command_executor(subscriber_repository, subscription_token_repository, dummy);

    // Act
    let actual = sut(command.clone()).await;

    // Assert
    assert!(actual.is_ok());

    let expected_name = command.as_subscribe().unwrap().name();
    let expected_email = command.as_subscribe().unwrap().email();
    let actual = find_subscriber_by_email(expected_email).await;
    assert_eq!(actual.name(), expected_name);
    assert_eq!(actual.email(), expected_email);
    assert!(matches!(actual.status(), Status::Pending));
}

#[rstest::rstest]
#[tokio::test]
async fn sut_generates_token_to_validate_email_address(
    #[future(awt)] subscriber_repository: SqlxSubscriberRepository,
    #[future(awt)] subscription_token_repository: SqlxSubscriptionTokenRepository,
    #[future(awt)]
    #[from(email_client_double)]
    dummy: EmailClientDouble,
    command: Command,
) {
    // Arrange
    let sut = new_command_executor(subscriber_repository, subscription_token_repository, dummy);

    // Act
    let _ = sut(command.clone()).await;

    // Assert
    let subscriber = find_subscriber_by_email(command.as_subscribe().unwrap().email()).await;
    let actual = find_subscription_token_by_subscriber_id(subscriber.id()).await;
    assert_eq!(subscriber.id(), actual.subscriber_id());
    assert!(!actual.token().is_empty());
}

#[rstest::rstest]
#[tokio::test]
async fn sut_generates_randomised_token_for_each_subscription(
    #[future(awt)] subscriber_repository: SqlxSubscriberRepository,
    #[future(awt)] subscription_token_repository: SqlxSubscriptionTokenRepository,
    #[future(awt)]
    #[from(email_client_double)]
    dummy: EmailClientDouble,
    commands: Vec<Command>,
) {
    // Arrange
    let sut = new_command_executor(subscriber_repository, subscription_token_repository, dummy);
    let mut tokens = Vec::new();

    // Act
    for command in commands {
        let _ = sut(command.clone()).await;

        let subscriber = find_subscriber_by_email(command.as_subscribe().unwrap().email()).await;
        let token = find_subscription_token_by_subscriber_id(subscriber.id())
            .await
            .token()
            .to_owned();
        tokens.push(token);
    }

    // Assert
    let set_of_tokens: HashSet<_> = HashSet::from_iter(tokens.clone());
    assert_eq!(set_of_tokens.len(), tokens.len());
}

#[rstest::rstest]
#[tokio::test]
async fn sut_raises_invalid_attributes_error_if_name_is_longer_than_256(
    #[future(awt)] subscriber_repository: SqlxSubscriberRepository,
    #[future(awt)] subscription_token_repository: SqlxSubscriptionTokenRepository,
    #[future(awt)]
    #[from(email_client_double)]
    dummy: EmailClientDouble,
    email: Email,
) {
    // Arrange
    let sut = new_command_executor(subscriber_repository, subscription_token_repository, dummy);
    let name = (0..(256..1024).fake::<u32>())
        .map(|_| "X")
        .collect::<String>();
    let command = Command::from(SubscribeCommand::new(name, email.as_ref().into()));

    // Act
    let actual = sut(command).await;

    // Assert
    assert!(actual.is_err());
    assert!(matches!(actual.unwrap_err(), Error::InvalidAttributes));
}

#[rstest::rstest]
#[tokio::test]
async fn sut_sends_sending_email_request_with_authorization_token_to_email_server_correctly(
    #[future(awt)] subscriber_repository: SqlxSubscriberRepository,
    #[future(awt)] subscription_token_repository: SqlxSubscriptionTokenRepository,
    #[future(awt)] email_server_and_client: (wiremock::MockServer, FakeEmailClient),
    command: Command,
) {
    // Arrange
    let (email_server, email_client) = email_server_and_client;
    let sut = new_command_executor(
        subscriber_repository,
        subscription_token_repository,
        email_client.clone(),
    );

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
    #[future(awt)] subscriber_repository: SqlxSubscriberRepository,
    #[future(awt)] subscription_token_repository: SqlxSubscriptionTokenRepository,
    #[future(awt)] email_server_and_client: (wiremock::MockServer, FakeEmailClient),
    command: Command,
) {
    // Arrange
    let (email_server, email_client) = email_server_and_client;
    let sut = new_command_executor(
        subscriber_repository,
        subscription_token_repository,
        email_client.clone(),
    );

    // Act
    let _ = sut(command.clone()).await;

    // Assert
    let subscriber = find_subscriber_by_email(command.as_subscribe().unwrap().email()).await;
    let subscription_token = find_subscription_token_by_subscriber_id(subscriber.id()).await;
    let request = extract_first_received_request(email_server).await;
    let actual: serde_json::Value = request.body_json().unwrap();
    let expected = serde_json::json!({
        "From": "test@gmail.com",
        "To": command.as_subscribe().unwrap().email(),
        "Subject": "hello!",
        "Content": format!("click this link: .. {} ..", subscription_token.token()).as_str(),
    });
    assert_eq!(actual, expected);
}

#[rstest::rstest]
#[tokio::test]
async fn sut_raises_failed_email_operation_error_if_email_server_responds_with_internal_server_error(
    #[future(awt)] subscriber_repository: SqlxSubscriberRepository,
    #[future(awt)] subscription_token_repository: SqlxSubscriptionTokenRepository,
    #[future(awt)] faulty_email_server_and_client: (wiremock::MockServer, FakeEmailClient),
    command: Command,
) {
    // Arrange
    let (_, email_client) = faulty_email_server_and_client;
    let sut = new_command_executor(
        subscriber_repository,
        subscription_token_repository,
        email_client.clone(),
    );

    // Act
    let actual = sut(command).await.unwrap_err();

    // Assert
    assert!(matches!(actual, Error::FailedEmailOperation));
}
