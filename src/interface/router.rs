use axum::extract::FromRef;
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

use crate::domain::subscriber::service::command::interface::ExecuteCommand as ExecuteSubscriberCommand;
use crate::interface::controllers;

#[derive(Clone)]
pub struct Container {
    execute_subscriber_command: ExecuteSubscriberCommand,
}

impl Container {
    pub fn new(execute_subscriber_command: ExecuteSubscriberCommand) -> Self {
        Self {
            execute_subscriber_command,
        }
    }
}

impl FromRef<Container> for ExecuteSubscriberCommand {
    fn from_ref(container: &Container) -> Self {
        container.execute_subscriber_command.clone()
    }
}

pub async fn get_router(container: Container) -> Router {
    Router::new()
        .route(
            "/subscriptions",
            post(controllers::post_subscriptions::control),
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
        .route("/healthz", get(controllers::get_healthz::control))
}
