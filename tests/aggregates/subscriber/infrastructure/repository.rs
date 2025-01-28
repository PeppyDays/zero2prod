use std::time::Duration;

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use zero2prod::aggregates::subscriber::infrastructure::repository::SqlxSubscriberRepository;
use zero2prod::aggregates::subscriber::infrastructure::repository::SqlxSubscriptionTokenRepository;
use zero2prod::configuration::get_configuration;
use zero2prod::configuration::Environment;

#[rstest::fixture]
pub async fn pool() -> Pool<Postgres> {
    let configuration = get_configuration(Environment::Test).unwrap();

    PgPoolOptions::new()
        .min_connections(2)
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect(
            configuration
                .subscriber
                .database
                .connection_string()
                .expose_secret(),
        )
        .await
        .unwrap()
}

#[rstest::fixture]
pub async fn subscriber_repository(
    #[future(awt)] pool: Pool<Postgres>,
) -> SqlxSubscriberRepository {
    SqlxSubscriberRepository::new(pool)
}

#[rstest::fixture]
pub async fn subscription_token_repository(
    #[future(awt)] pool: Pool<Postgres>,
) -> SqlxSubscriptionTokenRepository {
    SqlxSubscriptionTokenRepository::new(pool)
}
