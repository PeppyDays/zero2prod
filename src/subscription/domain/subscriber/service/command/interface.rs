use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::subscription::domain::subscriber::infrastructure::EmailClient;
use crate::subscription::domain::subscriber::infrastructure::Repository;
use crate::subscription::domain::subscriber::service::command::executors;
use crate::subscription::exception::Error;

pub enum Command {
    Subscribe(executors::subscribe::Command),
}

impl From<executors::subscribe::Command> for Command {
    fn from(command: executors::subscribe::Command) -> Self {
        Self::Subscribe(command)
    }
}

pub type ExecuteCommand =
    Arc<dyn Fn(Command) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> + Send + Sync>;

pub fn new_execute_command(
    repository: impl Repository,
    email_client: impl EmailClient,
) -> ExecuteCommand {
    Arc::new(move |command: Command| {
        let repository = repository.clone();
        let email_client = email_client.clone();

        Box::pin(async move {
            match command {
                Command::Subscribe(command) => {
                    executors::subscribe::execute(command, repository, email_client).await
                }
            }
        })
    })
}
