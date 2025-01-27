use crate::domain::subscriber::exception::Error;
use crate::domain::subscriber::model::Subscriber;
use crate::domain::subscriber::model::SubscriptionToken;

#[async_trait::async_trait]
pub trait SubscriberRepository: Send + Sync + Clone + 'static {
    async fn save(&self, subscriber: &Subscriber) -> Result<(), Error>;
}

#[async_trait::async_trait]
pub trait SubscriptionTokenRepository: Send + Sync + Clone + 'static {
    async fn save(&self, subscription_token: &SubscriptionToken) -> Result<(), Error>;
}

#[async_trait::async_trait]
pub trait EmailClient: Send + Sync + Clone + 'static {
    async fn send(&self, recipient: &Subscriber, subject: &str, content: &str)
        -> Result<(), Error>;
}
