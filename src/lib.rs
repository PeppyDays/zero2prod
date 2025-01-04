use std::io::Error;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

pub async fn check_health() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn run(listener: TcpListener) -> Result<(), Error> {
    let app = Router::new().route("/healthz", get(check_health));
    axum::serve(listener, app).await
}
