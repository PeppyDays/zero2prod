use std::io::Error;

use axum::extract::rejection::FormRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Form, Router};
use tokio::net::TcpListener;

async fn check_health() -> impl IntoResponse {
    StatusCode::OK
}

#[derive(Debug, serde::Deserialize)]
struct SubscriptionRequest {
    name: String,
    email: String,
}

async fn subscribe(form: Result<Form<SubscriptionRequest>, FormRejection>) -> impl IntoResponse {
    if form.is_err() {
        return StatusCode::BAD_REQUEST;
    }
    let Form(request) = form.unwrap();

    StatusCode::OK
}

pub async fn run(listener: TcpListener) -> Result<(), Error> {
    let app = Router::new()
        .route("/subscriptions", post(subscribe))
        .route("/healthz", get(check_health));

    axum::serve(listener, app).await
}
