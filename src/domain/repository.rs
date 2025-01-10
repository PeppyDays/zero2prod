use crate::domain::models::Subscriber;

#[async_trait::async_trait]
pub trait SubscriptionRepository: Send + Sync + 'static {
    async fn save(&self, subscriber: &Subscriber) -> Result<(), String>;
}
