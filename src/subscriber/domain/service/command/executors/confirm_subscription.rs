use crate::subscriber::domain::error::Error;
use crate::subscriber::domain::infrastructure::SubscriberRepository;
use crate::subscriber::domain::infrastructure::SubscriptionTokenRepository;

#[derive(Clone, Debug)]
pub struct Command {
    token: String,
}

impl Command {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

#[tracing::instrument(name = "Executing confirm subscription command", skip_all, fields(command = ?command))]
pub async fn execute(
    command: Command,
    subscriber_repository: impl SubscriberRepository,
    subscription_token_repository: impl SubscriptionTokenRepository,
) -> Result<(), Error> {
    let subscription_token = subscription_token_repository
        .find_by_token(command.token())
        .await?
        .ok_or(Error::TokenNotFound(command.token().into()))?;

    subscriber_repository
        .modify_by_id(subscription_token.subscriber_id(), |mut subscriber| {
            subscriber.confirm();
            subscriber
        })
        .await
}
