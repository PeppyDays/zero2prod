use std::error::Error;

use axum::extract::FromRef;
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use sqlx::Pool;
use sqlx::Postgres;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use uuid::ContextV7;
use uuid::Timestamp;
use uuid::Uuid;

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
        .route("/healthz", get(routes::health_check::check_health))
        .layer(
            // Refer to https://github.com/tokio-rs/axum/blob/main/examples/tracing-aka-logging/Cargo.toml
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                tracing::info_span!(
                    "Processing HTTP request",
                    method = ?request.method(),
                    path,
                    request_id = %Uuid::new_v7(Timestamp::now(ContextV7::new())),
                )
            }),
        );

    axum::serve(listener, app).await
}
