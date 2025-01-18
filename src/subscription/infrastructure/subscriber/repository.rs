use sqlx::Pool;
use sqlx::Postgres;

use crate::subscription::domain::subscriber::infrastructure::Repository;
use crate::subscription::domain::subscriber::model::Subscriber;
use crate::subscription::exception::Error;

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
        sqlx::query!(
            "INSERT INTO subscribers (id, name, email, subscribed_at) VALUES ($1, $2, $3, $4)",
            subscriber.id(),
            subscriber.name(),
            subscriber.email(),
            subscriber.subscribed_at().naive_utc(),
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
