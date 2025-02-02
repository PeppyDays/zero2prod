use std::error::Error;

use axum::extract::MatchedPath;
use axum::http::Request;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

use crate::subscriber;

pub async fn run(
    listener: TcpListener,
    subscriber_command_executor: impl subscriber::domain::service::CommandExecutor,
) -> Result<(), impl Error> {
    let subscriber_container =
        subscriber::interface::router::Container::new(subscriber_command_executor);
    let subscriber_router = subscriber::interface::router::get_router(subscriber_container).await;

    let app = Router::new()
        .merge(subscriber_router)
        .layer(
            // Refer to https://github.com/tokio-rs/axum/blob/main/examples/tracing-aka-logging/Cargo.toml
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                tracing::info_span!(
                    "Accepting HTTP request",
                    method = ?request.method(),
                    path,
                    request_id = %Uuid::now_v7(),
                )
            }),
        )
        .route("/healthz", get(|| async { StatusCode::OK }));

    axum::serve(listener, app).await
}
