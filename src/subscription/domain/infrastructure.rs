use crate::subscription::domain::model::Subscriber;
use crate::subscription::exception::Error;

#[async_trait::async_trait]
pub trait Repository: Send + Sync + 'static {
    async fn save(&self, subscriber: &Subscriber) -> Result<(), Error>;
}
