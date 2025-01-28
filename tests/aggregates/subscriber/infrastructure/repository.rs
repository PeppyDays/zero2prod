use sqlx::Pool;
use sqlx::Postgres;
use zero2prod::aggregates::subscriber::infrastructure::repository::SqlxSubscriberRepository;
use zero2prod::aggregates::subscriber::infrastructure::repository::SqlxSubscriptionTokenRepository;
use zero2prod::assembly::get_database_pool;
use zero2prod::configuration::get_configuration;
use zero2prod::configuration::Environment;

#[rstest::fixture]
pub async fn pool() -> Pool<Postgres> {
    let configuration = get_configuration(Environment::Test).unwrap();
    get_database_pool(&configuration.subscriber.database).await
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
