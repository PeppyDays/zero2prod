use std::error::Error;

use axum::extract::FromRef;
use axum::extract::MatchedPath;
use axum::extract::State;
use axum::http::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use axum::Form;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use uuid::ContextV7;
use uuid::Timestamp;
use uuid::Uuid;

use crate::subscription::domain::service::Command;
use crate::subscription::domain::service::CommandExecutor;

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
        .route("/subscriptions", post(control_post_subscription))
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
        .route("/healthz", get(control_get_healthz))
}

pub async fn run(
    listener: TcpListener,
    command_executor: CommandExecutor,
) -> Result<(), impl Error> {
    let container = Container::new(command_executor);
    let app = get_router(container).await;

    axum::serve(listener, app).await
}

pub async fn control_get_healthz() -> impl IntoResponse {
    StatusCode::OK
}

#[derive(serde::Deserialize)]
pub struct PostSubscriptionRequest {
    name: String,
    email: String,
}

#[tracing::instrument(name = "Adding a new subscriber", skip_all, fields(email = %request.email))]
pub async fn control_post_subscription(
    State(command_executor): State<CommandExecutor>,
    Form(request): Form<PostSubscriptionRequest>,
) -> impl IntoResponse {
    let command = Command::Subscribe {
        name: request.name,
        email: request.email,
    };

    match command_executor.execute(command).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
