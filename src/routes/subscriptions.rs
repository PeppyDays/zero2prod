use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Form;

// TODO: Implement the request usage and remove renaming of fields and remove the underscore
#[derive(Debug, serde::Deserialize)]
pub struct Request {
    #[serde(rename = "name")]
    _name: String,
    #[serde(rename = "email")]
    _email: String,
}

pub async fn subscribe(Form(_request): Form<Request>) -> impl IntoResponse {
    StatusCode::OK
}
