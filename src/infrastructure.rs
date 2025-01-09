use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::Subscriber;
use crate::domain::SubscriptionRepository;

pub struct SubscriptionSqlxRepository {
    pool: Pool<Postgres>,
}

impl SubscriptionSqlxRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SubscriptionRepository for SubscriptionSqlxRepository {
    #[tracing::instrument(name = "Saving subscriber details", skip_all)]
    async fn save(&self, subscriber: &Subscriber) -> Result<(), String> {
        sqlx::query!(
            "INSERT INTO subscriptions (id, name, email, subscribed_at) VALUES ($1, $2, $3, $4)",
            subscriber.id,
            subscriber.name,
            subscriber.email.as_ref(),
            subscriber.subscribed_at.naive_utc(),
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e.to_string()
        })?;

        Ok(())
    }
}
