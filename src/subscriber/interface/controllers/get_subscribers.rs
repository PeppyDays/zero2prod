use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::subscriber::domain::error::Error;
use crate::subscriber::domain::service::PluralQueryReader;
use crate::subscriber::domain::service::Query;

#[derive(Clone, Debug, serde::Serialize)]
pub struct Response {
    emails: Vec<String>,
}

pub async fn control(State(query_reader): State<Arc<dyn PluralQueryReader>>) -> impl IntoResponse {
    let query = Query::GetConfirmedSubscribers;
    match query_reader.read(query).await {
        Ok(subscribers) => {
            let response = Response {
                emails: subscribers
                    .iter()
                    .map(|subscriber| subscriber.email().into())
                    .collect(),
            };
            Ok(())
        }
        Err(error) => {
            tracing::error!("{}", error);
            Ok(())
        }
    }
}
