use std::time::Duration;

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use zero2prod::configuration::get_configuration;
use zero2prod::configuration::Environment;
use zero2prod::subscription::infrastructure::subscriber::repository::SqlxRepository;

#[rstest::fixture]
pub async fn repository() -> SqlxRepository {
    let configuration = get_configuration(Environment::Test).unwrap();

    let pool = PgPoolOptions::new()
        .min_connections(2)
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect(configuration.database.connection_string().expose_secret())
        .await
        .unwrap();

    SqlxRepository::new(pool)
}
