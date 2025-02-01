use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;
use zero2prod::assembly::get_database_pool;
use zero2prod::configuration::get_configuration;
use zero2prod::configuration::Environment;
use zero2prod::subscriber::domain::model::Subscriber;
use zero2prod::subscriber::domain::model::SubscriptionToken;
use zero2prod::subscriber::infrastructure::repository::SqlxSubscriberRepository;
use zero2prod::subscriber::infrastructure::repository::SqlxSubscriptionTokenRepository;
use zero2prod::subscriber::infrastructure::repository::SubscriberDataModel;
use zero2prod::subscriber::infrastructure::repository::SubscriptionTokenDataModel;

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

pub async fn find_subscriber_by_email(email: &str) -> Subscriber {
    let pool = pool().await;
    let row = sqlx::query!(
        "select id, name, email, subscribed_at, status from subscribers where email = $1",
        email,
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let data_model =
        SubscriberDataModel::new(row.id, row.name, row.email, row.subscribed_at, row.status);
    data_model.try_into().unwrap()
}

#[rstest::fixture]
pub async fn subscription_token_repository(
    #[future(awt)] pool: Pool<Postgres>,
) -> SqlxSubscriptionTokenRepository {
    SqlxSubscriptionTokenRepository::new(pool)
}

pub async fn find_subscription_token_by_subscriber_id(subscriber_id: &Uuid) -> SubscriptionToken {
    let pool = pool().await;
    let row = sqlx::query!(
        "select subscriber_id, token from subscription_tokens where subscriber_id = $1",
        subscriber_id,
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let data_model = SubscriptionTokenDataModel::new(row.token, row.subscriber_id);
    data_model.try_into().unwrap()
}
