use chrono::NaiveDateTime;
use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;

use crate::subscription::domain::subscriber::infrastructure::SubscriberRepository;
use crate::subscription::domain::subscriber::infrastructure::SubscriptionTokenRepository;
use crate::subscription::domain::subscriber::model::Subscriber;
use crate::subscription::domain::subscriber::model::SubscriptionToken;
use crate::subscription::exception::Error;

pub struct SubscriberDataModel {
    id: Uuid,
    name: String,
    email: String,
    subscribed_at: NaiveDateTime,
    status: String,
}

impl SubscriberDataModel {
    pub fn new(
        id: Uuid,
        name: String,
        email: String,
        subscribed_at: NaiveDateTime,
        status: String,
    ) -> Self {
        Self {
            id,
            name,
            email,
            subscribed_at,
            status,
        }
    }
}

impl TryFrom<SubscriberDataModel> for Subscriber {
    type Error = Error;

    fn try_from(data_model: SubscriberDataModel) -> Result<Self, Self::Error> {
        let name = data_model.name.as_str().try_into()?;
        let email = data_model.email.as_str().try_into()?;
        let subscribed_at = data_model.subscribed_at.and_utc();
        let status = data_model
            .status
            .as_str()
            .try_into()
            .map_err(|_| Error::InvalidAttributes)?;
        Ok(Subscriber::new(
            data_model.id,
            name,
            email,
            subscribed_at,
            status,
        ))
    }
}

impl From<&Subscriber> for SubscriberDataModel {
    fn from(entity: &Subscriber) -> Self {
        SubscriberDataModel {
            id: *entity.id(),
            name: entity.name().into(),
            email: entity.email().into(),
            subscribed_at: entity.subscribed_at().naive_utc(),
            status: entity.status().as_ref().into(),
        }
    }
}

#[derive(Clone)]
pub struct SqlxSubscriberRepository {
    pool: Pool<Postgres>,
}

impl SqlxSubscriberRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SubscriberRepository for SqlxSubscriberRepository {
    #[tracing::instrument(name = "Saving subscriber details", skip_all)]
    async fn save(&self, subscriber: &Subscriber) -> Result<(), Error> {
        let data_model: SubscriberDataModel = subscriber.into();
        sqlx::query!(
            "INSERT INTO subscribers (id, name, email, subscribed_at, status) VALUES ($1, $2, $3, $4, $5)",
            data_model.id,
            data_model.name,
            data_model.email,
            data_model.subscribed_at,
            data_model.status,
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

pub struct SubscriptionTokenDataModel {
    token: String,
    subscriber_id: Uuid,
}

impl SubscriptionTokenDataModel {
    pub fn new(token: String, subscriber_id: Uuid) -> Self {
        Self {
            token,
            subscriber_id,
        }
    }
}

impl TryFrom<SubscriptionTokenDataModel> for SubscriptionToken {
    type Error = Error;

    fn try_from(data_model: SubscriptionTokenDataModel) -> Result<Self, Self::Error> {
        Ok(SubscriptionToken::new(
            data_model.token,
            data_model.subscriber_id,
        ))
    }
}

impl From<&SubscriptionToken> for SubscriptionTokenDataModel {
    fn from(entity: &SubscriptionToken) -> Self {
        SubscriptionTokenDataModel::new(entity.token().into(), *entity.subscriber_id())
    }
}

#[derive(Clone)]
pub struct SqlxSubscriptionTokenRepository {
    pool: Pool<Postgres>,
}

impl SqlxSubscriptionTokenRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SubscriptionTokenRepository for SqlxSubscriptionTokenRepository {
    async fn save(&self, subscription_token: &SubscriptionToken) -> Result<(), Error> {
        let data_model: SubscriptionTokenDataModel = subscription_token.into();
        sqlx::query!(
            "INSERT INTO subscription_tokens (token, subscriber_id) VALUES ($1, $2)",
            data_model.token,
            data_model.subscriber_id,
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
