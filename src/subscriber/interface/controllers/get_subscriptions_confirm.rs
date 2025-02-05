use std::sync::Arc;

use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::subscriber::domain::error::Error;
use crate::subscriber::domain::service::CommandExecutor;
use crate::subscriber::domain::service::ConfirmSubscriptionCommand;
use crate::subscriber::interface::response::Response;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Request {
    token: String,
}

#[tracing::instrument(name = "Confirming subscription", skip_all, fields(request = ?request))]
pub async fn control(
    State(command_executor): State<Arc<dyn CommandExecutor>>,
    Query(request): Query<Request>,
) -> impl IntoResponse {
    let command = ConfirmSubscriptionCommand::new(request.token).into();

    match command_executor.execute(command).await {
        Ok(_) => Response::new(StatusCode::OK, None),
        Err(error) => {
            tracing::error!("{}", error);
            convert_error_to_response(error)
        }
    }
}

fn convert_error_to_response(error: Error) -> Response {
    match error {
        Error::InvariantViolated(_) => Response::new(
            StatusCode::BAD_REQUEST,
            Some("Failed to load the subscriber having the token.".into()),
        ),
        Error::TokenNotFound(_) => Response::new(
            StatusCode::NOT_FOUND,
            Some("Failed to find the token.".into()),
        ),
        Error::SubscriberNotFound(_) => Response::new(
            StatusCode::NOT_FOUND,
            Some("Failed to find a subscriber with the token.".into()),
        ),
        _ => Response::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("Failed to confirm subscription because of the unexpected system issue.".into()),
        ),
    }
}
