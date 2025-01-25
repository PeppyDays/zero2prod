use chrono::NaiveDateTime;
use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;

use crate::subscription::domain::subscriber::infrastructure::Repository;
use crate::subscription::domain::subscriber::model::Email;
use crate::subscription::domain::subscriber::model::Name;
use crate::subscription::domain::subscriber::model::Subscriber;
use crate::subscription::exception::Error;

pub struct SubscriberDataModel {
    id: Uuid,
    name: String,
    email: String,
    subscribed_at: NaiveDateTime,
}

impl SubscriberDataModel {
    pub fn new(id: Uuid, name: String, email: String, subscribed_at: NaiveDateTime) -> Self {
        Self {
            id,
            name,
            email,
            subscribed_at,
        }
    }
}

impl TryFrom<SubscriberDataModel> for Subscriber {
    type Error = Error;

    fn try_from(data_model: SubscriberDataModel) -> Result<Self, Self::Error> {
        let name: Name = data_model.name.as_str().try_into()?;
        let email: Email = data_model.email.as_str().try_into()?;
        let subscribed_at = data_model.subscribed_at.and_utc();
        Ok(Subscriber::new(data_model.id, name, email, subscribed_at))
    }
}

impl From<&Subscriber> for SubscriberDataModel {
    fn from(entity: &Subscriber) -> Self {
        SubscriberDataModel {
            id: *entity.id(),
            name: entity.name().into(),
            email: entity.email().into(),
            subscribed_at: entity.subscribed_at().naive_utc(),
        }
    }
}

#[derive(Clone)]
pub struct SqlxRepository {
    pool: Pool<Postgres>,
}

impl SqlxRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Repository for SqlxRepository {
    #[tracing::instrument(name = "Saving subscriber details", skip_all)]
    async fn save(&self, subscriber: &Subscriber) -> Result<(), Error> {
        let data_model: SubscriberDataModel = subscriber.into();
        sqlx::query!(
            "INSERT INTO subscribers (id, name, email, subscribed_at) VALUES ($1, $2, $3, $4)",
            data_model.id,
            data_model.name,
            data_model.email,
            data_model.subscribed_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            Error::FailedRepositoryOperation
        })?;

        Ok(())
    }
}
