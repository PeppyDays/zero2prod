use crate::subscription::domain::subscriber::model::Subscriber;
use crate::subscription::exception::Error;

#[async_trait::async_trait]
pub trait Repository: Send + Sync + Clone + 'static {
    async fn save(&self, subscriber: &Subscriber) -> Result<(), Error>;
}
