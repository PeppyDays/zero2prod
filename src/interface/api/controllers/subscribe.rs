use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Form;

use crate::domain::models::Email;
use crate::domain::models::Subscriber;
use crate::domain::repository::SubscriptionRepository;

#[derive(serde::Deserialize)]
pub struct Request {
    name: String,
    email: String,
}

#[tracing::instrument(name = "Adding a new subscriber", skip_all, fields(email = %request.email))]
pub async fn control(
    State(repository): State<Arc<dyn SubscriptionRepository>>,
    Form(request): Form<Request>,
) -> impl IntoResponse {
    let email: Email = request.email.try_into().unwrap();
    let subscriber = Subscriber::new(request.name, email);

    match repository.save(&subscriber).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}