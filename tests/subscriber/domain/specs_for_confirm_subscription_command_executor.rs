use uuid::Uuid;
use zero2prod::subscriber::domain::error::Error;
use zero2prod::subscriber::domain::infrastructure::SubscriberRepository;
use zero2prod::subscriber::domain::infrastructure::SubscriptionTokenRepository;
use zero2prod::subscriber::domain::model::Status;
use zero2prod::subscriber::domain::model::Subscriber;
use zero2prod::subscriber::domain::service::new_command_executor;
use zero2prod::subscriber::domain::service::Command;
use zero2prod::subscriber::infrastructure::repository::SqlxSubscriberRepository;
use zero2prod::subscriber::infrastructure::repository::SqlxSubscriptionTokenRepository;

use crate::subscriber::domain::model::subscriber;
use crate::subscriber::domain::model::subscription_token;
use crate::subscriber::domain::model::token;
use crate::subscriber::domain::service::confirm_subscription_command as command;
use crate::subscriber::domain::service::confirm_subscription_command;
use crate::subscriber::infrastructure::email_client::email_client_double;
use crate::subscriber::infrastructure::email_client::EmailClientDouble;
use crate::subscriber::infrastructure::repository::find_subscriber_by_email;
use crate::subscriber::infrastructure::repository::subscriber_repository;
use crate::subscriber::infrastructure::repository::subscription_token_repository;

#[rstest::rstest]
#[tokio::test]
async fn sut_changes_subscriber_status_as_confirmed_if_token_exists(
    #[future(awt)] subscriber_repository: SqlxSubscriberRepository,
    #[future(awt)] subscription_token_repository: SqlxSubscriptionTokenRepository,
    #[future(awt)]
    #[from(email_client_double)]
    dummy: EmailClientDouble,
    subscriber: Subscriber,
    token: String,
) {
    // Arrange
    let subscription_token = subscription_token(token.clone(), *subscriber.id());
    subscriber_repository.save(&subscriber).await.unwrap();
    subscription_token_repository
        .save(&subscription_token)
        .await
        .unwrap();

    let command = confirm_subscription_command(token);
    let sut = new_command_executor(subscriber_repository, subscription_token_repository, dummy);

    // Act
    sut(command).await.unwrap();

    // Assert
    let actual = find_subscriber_by_email(subscriber.email()).await;
    assert!(matches!(actual.status(), Status::Confirmed));
}

#[rstest::rstest]
#[tokio::test]
async fn sut_raises_token_not_found_error_if_token_does_not_exist(
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
    let actual = sut(command).await.unwrap_err();

    // Assert
    assert!(matches!(actual, Error::TokenNotFound));
}

#[rstest::rstest]
#[tokio::test]
async fn sut_raises_subscriber_not_found_error_if_subscriber_does_not_exist(
    #[future(awt)] subscriber_repository: SqlxSubscriberRepository,
    #[future(awt)] subscription_token_repository: SqlxSubscriptionTokenRepository,
    #[future(awt)]
    #[from(email_client_double)]
    dummy: EmailClientDouble,
    token: String,
) {
    // Arrange
    let subscription_token = subscription_token(token.clone(), Uuid::now_v7());
    subscription_token_repository
        .save(&subscription_token)
        .await
        .unwrap();

    let command = confirm_subscription_command(token);
    let sut = new_command_executor(subscriber_repository, subscription_token_repository, dummy);

    // Act
    let actual = sut(command).await.unwrap_err();

    // Assert
    assert!(matches!(actual, Error::SubscriberNotFound));
}
