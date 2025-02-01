use std::sync::Arc;

use tokio::sync::RwLock;
use zero2prod::aggregates::subscriber::domain::error::Error;
use zero2prod::aggregates::subscriber::domain::model::Email;
use zero2prod::aggregates::subscriber::domain::model::Name;
use zero2prod::aggregates::subscriber::domain::service::Command;
use zero2prod::aggregates::subscriber::domain::service::CommandExecutor;
use zero2prod::aggregates::subscriber::domain::service::ConfirmSubscriptionCommand;
use zero2prod::aggregates::subscriber::domain::service::SubscribeCommand;

use crate::aggregates::subscriber::domain::model::email;
use crate::aggregates::subscriber::domain::model::name;
use crate::aggregates::subscriber::domain::model::token;

#[rstest::fixture]
pub fn subscribe_command(name: Name, email: Email) -> Command {
    SubscribeCommand::new(name.as_ref().into(), email.as_ref().into()).into()
}

#[rstest::fixture]
pub fn subscribe_commands(#[default(5)] size: usize) -> Vec<Command> {
    (0..size)
        .map(|_| {
            Command::from(SubscribeCommand::new(
                name().as_ref().into(),
                email().as_ref().into(),
            ))
        })
        .collect()
}

#[rstest::fixture]
pub fn confirm_subscription_command(token: String) -> Command {
    ConfirmSubscriptionCommand::new(token).into()
}

#[derive(Clone)]
pub struct CommandExecutorSpy {
    command: Arc<RwLock<Option<Command>>>,
}

#[allow(clippy::new_without_default)]
impl CommandExecutorSpy {
    pub fn new() -> Self {
        CommandExecutorSpy {
            command: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn command(&self) -> Option<Command> {
        self.command.read().await.clone()
    }
}

#[async_trait::async_trait]
impl CommandExecutor for CommandExecutorSpy {
    async fn execute(&self, command: Command) -> Result<(), Error> {
        *self.command.write().await = Some(command);
        Ok(())
    }
}

#[rstest::fixture]
pub fn command_executor_spy() -> CommandExecutorSpy {
    CommandExecutorSpy::new()
}

#[derive(Clone)]
pub struct CommandExecutorStub {
    error: Arc<Option<Error>>,
}

#[allow(clippy::new_without_default)]
impl CommandExecutorStub {
    pub fn new(error: Option<Error>) -> Self {
        CommandExecutorStub {
            error: Arc::new(error),
        }
    }
}

#[async_trait::async_trait]
impl CommandExecutor for CommandExecutorStub {
    async fn execute(&self, _: Command) -> Result<(), Error> {
        match &*self.error {
            Some(error) => match error {
                Error::InvalidAttribute => Err(Error::InvalidAttribute),
                Error::CommandMismatched => Err(Error::CommandMismatched),
                Error::TokenNotFound => Err(Error::TokenNotFound),
                Error::SubscriberNotFound => Err(Error::SubscriberNotFound),
                Error::RepositoryOperationFailed => Err(Error::RepositoryOperationFailed),
                Error::EmailOperationFailed => Err(Error::EmailOperationFailed),
                Error::FailedUnexpectedly => Err(Error::FailedUnexpectedly),
            },
            None => Ok(()),
        }
    }
}

#[rstest::fixture]
pub fn command_executor_stub() -> CommandExecutorStub {
    CommandExecutorStub::new(None)
}

#[rstest::fixture]
pub fn faulty_command_executor_stub(
    #[default(Error::FailedUnexpectedly)] error: Error,
) -> CommandExecutorStub {
    CommandExecutorStub::new(Some(error))
}
