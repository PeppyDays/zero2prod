use fake::Fake;
use sqlx::Pool;
use sqlx::Postgres;
use zero2prod::subscription::domain::service::Command;
use zero2prod::subscription::domain::service::CommandExecutor;
use zero2prod::subscription::exception::Error;
use zero2prod::subscription::infrastructure::repository::SqlxRepository;

use crate::subscription::domain::command::subscribe_command;
use crate::subscription::infrastructure::connection::pool;
use crate::subscription::infrastructure::repository::repository;

#[rstest::rstest]
#[tokio::test]
async fn sut_stores_subscribers_correctly(
    #[future(awt)] repository: SqlxRepository,
    #[future(awt)] pool: Pool<Postgres>,
    subscribe_command: Command,
) {
    // Arrange
    let sut = CommandExecutor::new(repository);

    // Act
    let actual = sut.execute(subscribe_command).await;

    // Assert
    assert!(actual.is_ok());

    let x = sqlx::query!(
        "SELECT id, name, email, subscribed_at FROM subscribers WHERE email = $1",
        subscribe_command.email
    )
    .fetch_one(&pool)
    .await
    .unwrap();
}

#[rstest::rstest]
#[tokio::test]
async fn sut_raises_invalid_attributes_error_if_name_is_longer_than_256(
    #[future(awt)] repository: SqlxRepository,
    subscribe_command: Command,
) {
    // Arrange
    let sut = CommandExecutor::new(repository);

    let name = (0..(256..1024).fake::<u32>())
        .map(|_| "X")
        .collect::<String>();
    let command = replace_name_in_subscribe_command(subscribe_command, name);

    // Act
    let actual = sut.execute(command).await;

    // Assert
    assert!(actual.is_err());
    assert!(matches!(actual.unwrap_err(), Error::InvalidAttributes));
}

fn replace_name_in_subscribe_command(command: Command, name: String) -> Command {
    match command {
        Command::Subscribe { name: _, email } => Command::Subscribe { name, email },
    }
}
