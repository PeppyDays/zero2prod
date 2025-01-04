use axum::extract::rejection::FormRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Form;

#[derive(Debug, serde::Deserialize)]
pub struct Request {
    name: String,
    email: String,
}

pub async fn subscribe(form: Result<Form<Request>, FormRejection>) -> impl IntoResponse {
    if form.is_err() {
        return StatusCode::BAD_REQUEST;
    }
    let Form(_request) = form.unwrap();

    StatusCode::OK
}
