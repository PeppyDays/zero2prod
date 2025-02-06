use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;

use crate::subscriber::domain::service::PluralQueryReader;
use crate::subscriber::domain::service::Query;

#[derive(Clone, Debug, serde::Serialize)]
pub struct Response {
    subscribers: Vec<Subscribers>,
}

pub async fn control(State(query_reader): State<Arc<dyn PluralQueryReader>>) -> impl IntoResponse {
    let query = Query::GetConfirmedSubscribers;
    match query_reader.read(query).await {}
}
