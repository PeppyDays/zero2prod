use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use enum_as_inner::EnumAsInner;

use crate::subscriber::domain::error::Error;
use crate::subscriber::domain::infrastructure::EmailClient;
use crate::subscriber::domain::infrastructure::SubscriberRepository;
use crate::subscriber::domain::infrastructure::SubscriptionTokenRepository;
use crate::subscriber::domain::service::command::executors;

#[derive(Clone, EnumAsInner)]
pub enum Command {
    Subscribe(executors::subscribe::Command),
    ConfirmSubscription(executors::confirm_subscription::Command),
}

// TODO: Maybe good chance to learn macros with EnumAsInner and From
impl From<executors::subscribe::Command> for Command {
    fn from(command: executors::subscribe::Command) -> Self {
        Self::Subscribe(command)
    }
}

impl From<executors::confirm_subscription::Command> for Command {
    fn from(command: executors::confirm_subscription::Command) -> Self {
        Self::ConfirmSubscription(command)
    }
}

#[async_trait::async_trait]
pub trait CommandExecutor: Send + Sync + 'static {
    async fn execute(&self, command: Command) -> Result<(), Error>;
}

pub type CommandExecutorFuncion =
    Arc<dyn Fn(Command) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> + Send + Sync>;

#[async_trait::async_trait]
impl CommandExecutor for CommandExecutorFuncion {
    async fn execute(&self, command: Command) -> Result<(), Error> {
        self(command).await
    }
}

pub fn new_command_executor(
    subscriber_repository: impl SubscriberRepository,
    subscription_token_repository: impl SubscriptionTokenRepository,
    email_client: impl EmailClient,
) -> CommandExecutorFuncion {
    Arc::new(move |command| {
        let subscriber_repository = subscriber_repository.clone();
        let subscription_token_repository = subscription_token_repository.clone();
        let email_client = email_client.clone();

        Box::pin(async move {
            match command {
                Command::Subscribe(command) => {
                    executors::subscribe::execute(
                        command,
                        subscriber_repository,
                        subscription_token_repository,
                        email_client,
                    )
                    .await
                }
                Command::ConfirmSubscription(command) => {
                    executors::confirm_subscription::execute(
                        command,
                        subscriber_repository,
                        subscription_token_repository,
                    )
                    .await
                }
            }
        })
    })
}
