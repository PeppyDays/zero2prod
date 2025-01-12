use std::sync::Arc;

use crate::subscription::domain::infrastructure::Repository;
use crate::subscription::domain::model::Email;
use crate::subscription::domain::model::Name;
use crate::subscription::domain::model::Subscriber;
use crate::subscription::exception::Error;

pub enum Command {
    Subscribe { name: String, email: String },
}

#[derive(Clone)]
pub struct CommandExecutor {
    repository: Arc<dyn Repository>,
}

impl CommandExecutor {
    pub fn new(repository: impl Repository) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }

    pub async fn execute(&self, command: Command) -> Result<(), Error> {
        match command {
            Command::Subscribe { name, email } => {
                let name: Name = name.try_into().map_err(|_| Error::InvalidAttributes)?;
                let email: Email = email.try_into().map_err(|_| Error::InvalidAttributes)?;
                let subscriber = Subscriber::new(name, email);

                self.repository
                    .save(&subscriber)
                    .await
                    .map_err(|_| Error::RepositoryOperationFailed)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod subscribe_command_executor {
        use std::sync::Mutex;

        use fake::faker::internet::en::SafeEmail;
        use fake::Fake;

        use crate::subscription::domain::infrastructure::Repository;
        use crate::subscription::domain::model::Subscriber;
        use crate::subscription::domain::service::Command;
        use crate::subscription::domain::service::CommandExecutor;
        use crate::subscription::exception::Error;

        struct FakeRepository {
            subscribers: Mutex<Vec<Subscriber>>,
        }

        impl FakeRepository {
            fn new() -> Self {
                Self {
                    subscribers: Mutex::new(Vec::new()),
                }
            }
        }

        #[async_trait::async_trait]
        impl Repository for FakeRepository {
            async fn save(&self, subscriber: &Subscriber) -> Result<(), Error> {
                self.subscribers.lock().unwrap().push(subscriber.clone());
                Ok(())
            }
        }

        #[rstest::rstest]
        #[tokio::test]
        async fn sut_raises_invalid_attributes_error_if_name_is_longer_than_256() {
            // Arrange
            let command_executor = CommandExecutor::new(FakeRepository::new());

            let name = (0..(256..1024).fake::<u32>())
                .map(|_| "X")
                .collect::<String>();
            let email = SafeEmail().fake();
            let command = Command::Subscribe { name, email };

            // Act
            let actual = command_executor.execute(command).await;

            // Assert
            assert!(actual.is_err());
            assert!(matches!(actual.unwrap_err(), Error::InvalidAttributes));
        }
    }

    // #[rstest::rstest]
    // #[test]
    // #[case(name())]
    // fn name_is_created_correctly(#[case] name: String) {
    //     // Act
    //     let actual: Name = name.clone().try_into().unwrap();

    //     // Assert
    //     assert_eq!(actual.as_ref(), name.as_str());
    // }

    // #[rstest::rstest]
    // #[case("  ")]
    // #[case("\t\t")]
    // #[test]
    // fn name_does_not_include_pre_and_post_whitespaces(#[case] name: String) {
    //     // Act
    //     let actual: Result<Name, &'static str> = name.try_into();

    //     // Assert
    //     assert!(actual.is_err());
    // }

    // #[rstest::rstest]
    // #[case((0..(256..1024).fake::<u32>()).map(|_| "X").collect::<String>())]
    // #[test]
    // fn name_should_be_shorter_than_256(#[case] name: String) {
    //     // Act
    //     let actual: Result<Name, &'static str> = name.try_into();

    //     // Assert
    //     assert!(actual.is_err());
    // }

    // #[rstest::rstest]
    // #[case(format!("{}{}{}", name(), "/", name()))]
    // #[case(format!("{}{}{}", name(), "(", name()))]
    // #[case(format!("{}{}{}", name(), ")", name()))]
    // #[case(format!("{}{}{}", name(), "\"", name()))]
    // #[case(format!("{}{}{}", name(), "<", name()))]
    // #[case(format!("{}{}{}", name(), ">", name()))]
    // #[case(format!("{}{}{}", name(), "\\", name()))]
    // #[case(format!("{}{}{}", name(), "{", name()))]
    // #[case(format!("{}{}{}", name(), "}", name()))]
    // #[case(format!("{}{}{}", name(), "?", name()))]
    // #[case(format!("{}{}{}", name(), "%", name()))]
    // #[test]
    // fn name_should_not_include_forbidden_characters(#[case] name: String) {
    //     // Act
    //     let actual: Result<Name, &'static str> = name.try_into();

    //     // Assert
    //     assert!(actual.is_err());
    // }

    // fn name() -> String {
    //     FakeName().fake()
    // }
}
