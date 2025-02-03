use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Form;

use crate::subscriber::domain::service::CommandExecutor;
use crate::subscriber::domain::service::SubscribeCommand;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Request {
    name: String,
    email: String,
}

#[tracing::instrument(name = "Adding a new subscriber", skip_all, fields(request = ?request))]
pub async fn control(
    State(command_executor): State<Arc<dyn CommandExecutor>>,
    Form(request): Form<Request>,
) -> impl IntoResponse {
    let command = SubscribeCommand::new(request.name, request.email).into();

    match command_executor.execute(command).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
