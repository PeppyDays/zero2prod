use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Form;

use crate::aggregates::subscriber::domain::service::command::executors::subscribe::Command as SubscribeCommand;
use crate::aggregates::subscriber::domain::service::command::interface::ExecuteCommand as ExecuteSubscriberCommand;

#[derive(serde::Deserialize)]
pub struct Request {
    name: String,
    email: String,
}

#[tracing::instrument(name = "Adding a new subscriber", skip_all, fields(email = %request.email))]
pub async fn control(
    State(execute_subscriber_command): State<ExecuteSubscriberCommand>,
    Form(request): Form<Request>,
) -> impl IntoResponse {
    let command = SubscribeCommand::new(request.name, request.email).into();

    match execute_subscriber_command(command).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
