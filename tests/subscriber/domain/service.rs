use std::sync::Arc;

use anyhow::anyhow;
use tokio::sync::RwLock;
use zero2prod::subscriber::domain::error::Error;
use zero2prod::subscriber::domain::model::Email;
use zero2prod::subscriber::domain::model::Name;
use zero2prod::subscriber::domain::service::Command;
use zero2prod::subscriber::domain::service::CommandExecutor;
use zero2prod::subscriber::domain::service::ConfirmSubscriptionCommand;
use zero2prod::subscriber::domain::service::SubscribeCommand;

use crate::subscriber::domain::model::email;
use crate::subscriber::domain::model::name;
use crate::subscriber::domain::model::token;

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
        let error = &*self.error;
        if let Some(error) = error {
            return match error {
                Error::InvariantViolated(message) => Err(Error::InvariantViolated(message.into())),
                Error::TokenNotFound(message) => Err(Error::TokenNotFound(message.into())),
                Error::SubscriberNotFound(id) => Err(Error::SubscriberNotFound(*id)),
                Error::RepositoryOperationFailed(_) => {
                    Err(Error::RepositoryOperationFailed(anyhow!("")))
                }
                Error::EmailOperationFailed(_) => Err(Error::EmailOperationFailed(anyhow!(""))),
                Error::FailedUnexpectedly(_) => Err(Error::FailedUnexpectedly(anyhow!(""))),
            };
        };
        Ok(())
    }
}

#[rstest::fixture]
pub fn command_executor_stub() -> CommandExecutorStub {
    CommandExecutorStub::new(None)
}

#[rstest::fixture]
pub fn faulty_command_executor_stub(
    #[default(Error::FailedUnexpectedly(anyhow!("")))] error: Error,
) -> CommandExecutorStub {
    CommandExecutorStub::new(Some(error))
}
