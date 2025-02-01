use std::sync::Arc;

use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::aggregates::subscriber::domain::error::Error;
use crate::aggregates::subscriber::domain::service::CommandExecutor;
use crate::aggregates::subscriber::domain::service::ConfirmSubscriptionCommand;

#[derive(serde::Deserialize)]
pub struct Request {
    token: String,
}

pub async fn control(
    State(subscriber_command_executor): State<Arc<dyn CommandExecutor>>,
    Query(request): Query<Request>,
) -> impl IntoResponse {
    let command = ConfirmSubscriptionCommand::new(request.token).into();

    match subscriber_command_executor.execute(command).await {
        Ok(_) => StatusCode::OK,
        Err(error) => match error {
            Error::InvalidAttribute => StatusCode::BAD_REQUEST,
            Error::CommandMismatched => StatusCode::BAD_REQUEST,
            Error::TokenNotFound => StatusCode::NOT_FOUND,
            Error::SubscriberNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        },
    }
}
