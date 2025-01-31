use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Form;

use crate::aggregates::subscriber::domain::service::CommandExecutor as SubscriberCommandExecutor;
use crate::aggregates::subscriber::domain::service::SubscribeCommand;

#[derive(serde::Deserialize)]
pub struct Request {
    name: String,
    email: String,
}

#[tracing::instrument(name = "Adding a new subscriber", skip_all, fields(email = %request.email))]
pub async fn control(
    State(subscriber_command_executor): State<Arc<dyn SubscriberCommandExecutor>>,
    Form(request): Form<Request>,
) -> impl IntoResponse {
    let command = SubscribeCommand::new(request.name, request.email).into();

    match subscriber_command_executor.execute(command).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
