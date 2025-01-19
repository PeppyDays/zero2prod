use claims::assert_err;
use claims::assert_matches;
use claims::assert_ok;
use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use zero2prod::subscription::domain::subscriber::service::command::executors::subscribe::Command as SubscribeCommand;
use zero2prod::subscription::domain::subscriber::service::command::interface::new_execute_command;
use zero2prod::subscription::domain::subscriber::service::command::interface::Command;
use zero2prod::subscription::exception::Error;
use zero2prod::subscription::infrastructure::subscriber::repository::SqlxRepository;

use crate::subscription::domain::subscriber::command::subscribe_command;
use crate::subscription::infrastructure::subscriber::repository::repository;

#[rstest::rstest]
#[tokio::test]
async fn sut_stores_subscribers_correctly(
    #[future(awt)] repository: SqlxRepository,
    subscribe_command: Command,
) {
    // Arrange
    let sut = new_execute_command(repository);

    // Act
    let actual = sut(subscribe_command).await;

    // Assert
    assert_ok!(actual);
}

#[rstest::rstest]
#[tokio::test]
async fn sut_raises_invalid_attributes_error_if_name_is_longer_than_256(
    #[future(awt)] repository: SqlxRepository,
) {
    // Arrange
    let sut = new_execute_command(repository);
    let name = (0..(256..1024).fake::<u32>())
        .map(|_| "X")
        .collect::<String>();

    let command = Command::Subscribe(SubscribeCommand::new(name, SafeEmail().fake()));

    // Act
    let actual = sut(command).await;

    // Assert
    assert_err!(&actual);
    assert_matches!(&actual.unwrap_err(), Error::InvalidAttributes);
}
