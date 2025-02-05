use crate::subscriber::domain::error::Error;
use crate::subscriber::domain::infrastructure::SubscriberRepository;
use crate::subscriber::domain::model::Status;
use crate::subscriber::domain::model::Subscriber;

pub async fn read(
    subscriber_repository: impl SubscriberRepository,
) -> Result<Vec<Subscriber>, Error> {
    subscriber_repository
        .find_by_status(Status::Confirmed)
        .await
}
