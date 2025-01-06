use std::error::Error;

use axum::extract::FromRef;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use sqlx::Pool;
use sqlx::Postgres;
use tokio::net::TcpListener;

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub database_pool: Pool<Postgres>,
}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Self {
        state.database_pool.clone()
    }
}

pub async fn run(listener: TcpListener, state: AppState) -> Result<(), impl Error> {
    let app = Router::new()
        .route("/subscriptions", post(routes::subscriptions::subscribe))
        .with_state(state)
        .route("/healthz", get(routes::health_check::check_health));

    axum::serve(listener, app).await
}
