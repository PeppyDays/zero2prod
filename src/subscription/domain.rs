use std::convert::TryFrom;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use uuid::ContextV7;
use uuid::Timestamp;
use uuid::Uuid;

use crate::subscription::exception::Error;

pub struct Subscriber {
    id: Uuid,
    name: Name,
    email: Email,
    subscribed_at: DateTime<Utc>,
}

impl Subscriber {
    pub fn new(name: Name, email: Email) -> Self {
        Self {
            id: Uuid::new_v7(Timestamp::now(ContextV7::new())),
            name,
            email,
            subscribed_at: Utc::now(),
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn email(&self) -> &str {
        self.email.as_ref()
    }

    pub fn subscribed_at(&self) -> &DateTime<Utc> {
        &self.subscribed_at
    }
}

const FORBIDDEN_CHARACTERS: [char; 11] = ['/', '(', ')', '\"', '<', '>', '\\', '{', '}', '?', '%'];

pub struct Name(String);

impl Name {
    fn validate(name: &str) -> Result<(), Error> {
        if name.trim().is_empty() {
            return Err(Error::InvalidAttributes);
        }

        if name.len() >= 256 {
            return Err(Error::InvalidAttributes);
        }

        if name.chars().any(|c| FORBIDDEN_CHARACTERS.contains(&c)) {
            return Err(Error::InvalidAttributes);
        }

        Ok(())
    }
}

impl TryFrom<String> for Name {
    type Error = Error;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Name::validate(name.as_str()).map(|_| Name(name))
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct Email(String);

impl Email {
    fn validate(email: &str) -> Result<(), Error> {
        if email.len() < 5 || !email.contains('@') {
            return Err(Error::InvalidAttributes);
        }
        Ok(())
    }
}

impl TryFrom<String> for Email {
    type Error = Error;

    fn try_from(email: String) -> Result<Self, Self::Error> {
        Email::validate(email.as_str()).map(|_| Email(email))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[async_trait::async_trait]
pub trait Repository: Send + Sync + 'static {
    async fn save(&self, subscriber: &Subscriber) -> Result<(), Error>;
}

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

        use crate::subscription::domain::Command;
        use crate::subscription::domain::CommandExecutor;
        use crate::subscription::domain::Email;
        use crate::subscription::domain::Name;
        use crate::subscription::domain::Repository;
        use crate::subscription::domain::Subscriber;
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
                let subscriber = Subscriber {
                    id: *subscriber.id(),
                    name: Name(subscriber.name.as_ref().into()),
                    email: Email(subscriber.email.as_ref().into()),
                    subscribed_at: *subscriber.subscribed_at(),
                };
                self.subscribers.lock().unwrap().push(subscriber);
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
