use crate::subscription::domain::subscriber::infrastructure::EmailClient;
use crate::subscription::domain::subscriber::infrastructure::SubscriberRepository;
use crate::subscription::domain::subscriber::infrastructure::SubscriptionTokenRepository;
use crate::subscription::domain::subscriber::model::Subscriber;
use crate::subscription::domain::subscriber::model::SubscriptionToken;
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
    subscriber_repository: impl SubscriberRepository,
    subscription_token_repository: impl SubscriptionTokenRepository,
    email_client: impl EmailClient,
) -> Result<(), Error> {
    let subscriber = Subscriber::create(&command.name, &command.email)?;
    subscriber_repository.save(&subscriber).await?;

    let subscription_token = SubscriptionToken::create(*subscriber.id());
    subscription_token_repository
        .save(&subscription_token)
        .await?;

    email_client
        .send(
            &subscriber,
            "hello!",
            format!("click this link: .. {} ..", subscription_token.token()).as_ref(),
        )
        .await
}
