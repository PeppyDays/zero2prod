use crate::subscription::domain::subscriber::infrastructure::Repository;
use crate::subscription::domain::subscriber::model::Subscriber;
use crate::subscription::exception::Error;

pub struct Command {
    name: String,
    email: String,
}

impl Command {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }
}

pub async fn execute(command: Command, repository: impl Repository) -> Result<(), Error> {
    let subscriber = Subscriber::create(&command.name, &command.email)?;

    repository
        .save(&subscriber)
        .await
        .map_err(|_| Error::FailedRepositoryOperation)
}
