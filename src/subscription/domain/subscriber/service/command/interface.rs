use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use enum_as_inner::EnumAsInner;

use crate::subscription::domain::subscriber::infrastructure::EmailClient;
use crate::subscription::domain::subscriber::infrastructure::SubscriberRepository;
use crate::subscription::domain::subscriber::infrastructure::SubscriptionTokenRepository;
use crate::subscription::domain::subscriber::service::command::executors;
use crate::subscription::exception::Error;

#[derive(Clone, EnumAsInner)]
pub enum Command {
    Subscribe(executors::subscribe::Command),
}

// TODO: Maybe good chance to learn macros with EnumAsInner and From
impl From<executors::subscribe::Command> for Command {
    fn from(command: executors::subscribe::Command) -> Self {
        Self::Subscribe(command)
    }
}

pub type ExecuteCommand =
    Arc<dyn Fn(Command) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> + Send + Sync>;

pub fn new_execute_command(
    subscriber_repository: impl SubscriberRepository,
    subscription_token_repository: impl SubscriptionTokenRepository,
    email_client: impl EmailClient,
) -> ExecuteCommand {
    Arc::new(move |command: Command| {
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
            }
        })
    })
}
