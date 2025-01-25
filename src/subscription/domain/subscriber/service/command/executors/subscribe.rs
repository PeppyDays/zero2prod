use crate::subscription::domain::subscriber::infrastructure::EmailClient;
use crate::subscription::domain::subscriber::infrastructure::Repository;
use crate::subscription::domain::subscriber::model::Subscriber;
use crate::subscription::exception::Error;

#[derive(Clone)]
pub struct Command {
    name: String,
    email: String,
}

impl Command {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn email(&self) -> &str {
        self.email.as_str()
    }
}

pub async fn execute(
    command: Command,
    repository: impl Repository,
    email_client: impl EmailClient,
) -> Result<(), Error> {
    let subscriber = Subscriber::create(&command.name, &command.email)?;

    repository.save(&subscriber).await?;

    email_client
        .send(&subscriber, "hello!", "click this link: ..")
        .await
}
