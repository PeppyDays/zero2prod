use reqwest::StatusCode;
use zero2prod::subscriber::domain::error::Error;
use zero2prod::subscriber::domain::service::ConfirmSubscriptionCommand;

use crate::interface::system::SystemSurface;
use crate::subscriber::domain::model::token;
use crate::subscriber::domain::service::command_executor_spy;
use crate::subscriber::domain::service::faulty_command_executor_stub;
use crate::subscriber::domain::service::CommandExecutorSpy;
use crate::subscriber::domain::service::CommandExecutorStub;

#[rstest::rstest]
#[tokio::test]
async fn sut_delivers_confirm_subscription_command_to_command_executor_correctly(
    command_executor_spy: CommandExecutorSpy,
    token: String,
) {
    // Arrange
    let sut = SystemSurface::new(command_executor_spy.clone()).await;

    // Act
    let _ = sut
        .requestor
        .get_subscriptions_confirm(Some(token.clone()))
        .await;

    // Assert
    let actual = parse_confirm_subscription_command(&command_executor_spy).await;
    let expected = ConfirmSubscriptionCommand::new(token);
    assert_eq_confirm_subscription_command(actual, expected);
}

#[rstest::rstest]
#[tokio::test]
async fn sut_responds_status_ok_if_token_is_valid(
    command_executor_spy: CommandExecutorSpy,
    token: String,
) {
    // Arrange
    let sut = SystemSurface::new(command_executor_spy.clone()).await;

    // Act
    let _ = sut
        .requestor
        .get_subscriptions_confirm(Some(token.clone()))
        .await;

    // Assert
    let actual = parse_confirm_subscription_command(&command_executor_spy).await;
    let expected = ConfirmSubscriptionCommand::new(token);
    assert_eq_confirm_subscription_command(actual, expected);
}

#[rstest::rstest]
#[tokio::test]
async fn sut_responds_status_bad_request_if_token_is_missing(
    #[from(command_executor_spy)] command_executor_dummy: CommandExecutorSpy,
) {
    // Arrange
    let sut = SystemSurface::new(command_executor_dummy).await;

    // Act
    let response = sut.requestor.get_subscriptions_confirm(None).await;

    // Assert
    let actual = response.status();
    assert!(matches!(actual, StatusCode::BAD_REQUEST));
}

#[rstest::rstest]
#[tokio::test]
async fn sut_responds_status_not_found_if_token_does_not_exist(
    #[with(Error::TokenNotFound)]
    #[from(faulty_command_executor_stub)]
    command_executor_stub: CommandExecutorStub,
    token: String,
) {
    // Arrange
    let sut = SystemSurface::new(command_executor_stub).await;

    // Act
    let response = sut.requestor.get_subscriptions_confirm(Some(token)).await;

    // Assert
    let actual = response.status();
    assert!(matches!(actual, StatusCode::NOT_FOUND));
}

#[rstest::rstest]
#[tokio::test]
async fn sut_responds_status_not_found_if_subscriber_inferred_from_token_does_not_exist(
    #[with(Error::SubscriberNotFound)]
    #[from(faulty_command_executor_stub)]
    command_executor_stub: CommandExecutorStub,
    token: String,
) {
    // Arrange
    let sut = SystemSurface::new(command_executor_stub).await;

    // Act
    let response = sut.requestor.get_subscriptions_confirm(Some(token)).await;

    // Assert
    let actual = response.status();
    assert!(matches!(actual, StatusCode::NOT_FOUND));
}

async fn parse_confirm_subscription_command(
    spy: &CommandExecutorSpy,
) -> ConfirmSubscriptionCommand {
    spy.command()
        .await
        .unwrap()
        .as_confirm_subscription()
        .unwrap()
        .clone()
}

fn assert_eq_confirm_subscription_command(
    actual: ConfirmSubscriptionCommand,
    expected: ConfirmSubscriptionCommand,
) -> bool {
    actual.token() == expected.token()
}
