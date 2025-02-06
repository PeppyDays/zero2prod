use uuid::Uuid;

use crate::subscriber::domain::error::Error;
use crate::subscriber::domain::model::Status;
use crate::subscriber::domain::model::Subscriber;
use crate::subscriber::domain::model::SubscriptionToken;

#[async_trait::async_trait]
pub trait SubscriberRepository: Send + Sync + Clone + 'static {
    async fn find_by_status(&self, status: Status) -> Result<Vec<Subscriber>, Error>;
    async fn save(&self, subscriber: &Subscriber) -> Result<(), Error>;
    async fn modify_by_id<F>(&self, id: &Uuid, modifier: F) -> Result<(), Error>
    where
        F: FnOnce(Subscriber) -> Subscriber + Send + Sync;
}

#[async_trait::async_trait]
pub trait SubscriptionTokenRepository: Send + Sync + Clone + 'static {
    async fn save(&self, subscription_token: &SubscriptionToken) -> Result<(), Error>;
    async fn find_by_token(&self, token: &str) -> Result<Option<SubscriptionToken>, Error>;
}

#[async_trait::async_trait]
pub trait EmailClient: Send + Sync + Clone + 'static {
    async fn send(&self, recipient: &Subscriber, subject: &str, content: &str)
        -> Result<(), Error>;
}
