use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Form;

use crate::subscriber::domain::error::Error;
use crate::subscriber::domain::service::CommandExecutor;
use crate::subscriber::domain::service::SubscribeCommand;
use crate::subscriber::interface::response::Response;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Request {
    name: String,
    email: String,
}

#[tracing::instrument(name = "Registering a new subscriber", skip_all, fields(request = ?request))]
pub async fn control(
    State(command_executor): State<Arc<dyn CommandExecutor>>,
    Form(request): Form<Request>,
) -> impl IntoResponse {
    let command = SubscribeCommand::new(request.name, request.email).into();

    match command_executor.execute(command).await {
        Ok(_) => Response::new(StatusCode::OK, None),
        Err(error) => {
            tracing::error!("{:?}", error);
            convert_error_to_response(error)
        }
    }
}

fn convert_error_to_response(error: Error) -> Response {
    match error {
        Error::InvariantViolated(message) => Response::new(StatusCode::BAD_REQUEST, Some(message)),
        _ => Response::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some(
                "Failed to register a new subscriber because of the unexpected system issue."
                    .into(),
            ),
        ),
    }
}
