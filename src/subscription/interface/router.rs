use axum::extract::FromRef;
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

use crate::subscription::domain::service::CommandExecutor;
use crate::subscription::interface::controller;

#[derive(Clone)]
pub struct Container {
    command_executor: CommandExecutor,
}

impl Container {
    pub fn new(command_executor: CommandExecutor) -> Self {
        Self { command_executor }
    }
}

impl FromRef<Container> for CommandExecutor {
    fn from_ref(container: &Container) -> Self {
        container.command_executor.clone()
    }
}

pub async fn get_router(container: Container) -> Router {
    Router::new()
        .route(
            "/subscriptions",
            post(controller::post_subscriptions::control),
        )
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
                    request_id = %Uuid::now_v7(),
                )
            }),
        )
        .route("/healthz", get(controller::get_healthz::control))
}
