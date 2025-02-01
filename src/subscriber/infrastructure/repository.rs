use chrono::NaiveDateTime;
use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;

use crate::subscriber::domain::error::Error;
use crate::subscriber::domain::infrastructure::SubscriberRepository;
use crate::subscriber::domain::infrastructure::SubscriptionTokenRepository;
use crate::subscriber::domain::model::Subscriber;
use crate::subscriber::domain::model::SubscriptionToken;

#[derive(Debug)]
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
            .map_err(|_| Error::InvalidAttribute)?;
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
            Error::RepositoryOperationFailed
        })?;

        Ok(())
    }

    async fn modify_by_id<F>(&self, id: &Uuid, modifier: F) -> Result<(), Error>
    where
        F: FnOnce(Subscriber) -> Subscriber + Send + Sync,
    {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| Error::RepositoryOperationFailed)?;

        let subscriber = sqlx::query!(
                "SELECT id, name, email, subscribed_at, status FROM subscribers WHERE id = $1 FOR UPDATE",
                id,
            )
            .fetch_one(&mut *transaction)
            .await
            .map(|r| SubscriberDataModel::new(
                r.id,
                r.name,
                r.email,
                r.subscribed_at,
                r.status,
            ))
            .map_err(|e| {
                match e {
                    sqlx::Error::RowNotFound => Error::SubscriberNotFound,
                    _ => Error::RepositoryOperationFailed,
                }
            })?
            .try_into()?;

        let modified_data_model: SubscriberDataModel = (&modifier(subscriber)).into();

        sqlx::query!(
            "UPDATE subscribers SET name = $1, email = $2, status = $3 WHERE id = $4",
            modified_data_model.name,
            modified_data_model.email,
            modified_data_model.status,
            modified_data_model.id,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|_| Error::RepositoryOperationFailed)?;

        transaction
            .commit()
            .await
            .map_err(|_| Error::RepositoryOperationFailed)
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
            Error::RepositoryOperationFailed
        })?;
        Ok(())
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<SubscriptionToken>, Error> {
        sqlx::query!(
            "SELECT token, subscriber_id FROM subscription_tokens WHERE token = $1",
            token,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| Error::RepositoryOperationFailed)?
        .map(|r| SubscriptionTokenDataModel::new(r.token, r.subscriber_id).try_into())
        .transpose()
        .map_err(|_| Error::InvalidAttribute)
    }
}
