use std::sync::Arc;

use axum::extract::FromRef;
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use tower_http::trace::TraceLayer;
use uuid::ContextV7;
use uuid::Timestamp;
use uuid::Uuid;

use crate::domain::repository::SubscriptionRepository;
use crate::interface::api::controllers;

#[derive(Clone)]
pub struct Container {
    repository: Arc<dyn SubscriptionRepository>,
}

impl Container {
    pub(crate) fn new(repository: impl SubscriptionRepository) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }
}

impl FromRef<Container> for Arc<dyn SubscriptionRepository> {
    fn from_ref(state: &Container) -> Self {
        state.repository.clone()
    }
}

pub async fn get_router(container: Container) -> Router {
    Router::new()
        .route("/subscriptions", post(controllers::subscribe::control))
        .with_state(container)
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
        )
        .route("/healthz", get(controllers::check_health::control))
}
