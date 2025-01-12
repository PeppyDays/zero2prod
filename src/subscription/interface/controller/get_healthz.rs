use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn control() -> impl IntoResponse {
    StatusCode::OK
}
