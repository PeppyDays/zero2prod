use std::time::Duration;

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use zero2prod::configuration::get_configuration;
use zero2prod::configuration::Environment;
use zero2prod::subscription::infrastructure::subscriber::repository::SqlxRepository;

#[rstest::fixture]
pub async fn pool() -> Pool<Postgres> {
    let configuration = get_configuration(Environment::Test).unwrap();

    PgPoolOptions::new()
        .min_connections(2)
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect(configuration.database.connection_string().expose_secret())
        .await
        .unwrap()
}

#[rstest::fixture]
pub async fn repository(#[future(awt)] pool: Pool<Postgres>) -> SqlxRepository {
    SqlxRepository::new(pool)
}
