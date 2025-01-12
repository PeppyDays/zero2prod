use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Form;

use crate::subscription::domain::service::Command;
use crate::subscription::domain::service::CommandExecutor;

#[derive(serde::Deserialize)]
pub struct Request {
    name: String,
    email: String,
}

#[tracing::instrument(name = "Adding a new subscriber", skip_all, fields(email = %request.email))]
pub async fn control(
    State(command_executor): State<CommandExecutor>,
    Form(request): Form<Request>,
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
