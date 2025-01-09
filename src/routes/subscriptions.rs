use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Form;
use chrono::Utc;
use sqlx::types::chrono;
use sqlx::Pool;
use sqlx::Postgres;
use uuid::ContextV7;
use uuid::Timestamp;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct Request {
    name: String,
    email: String,
}

#[tracing::instrument(name = "Adding a new subscriber", skip_all, fields(email = %request.email))]
pub async fn subscribe(
    State(pool): State<Pool<Postgres>>,
    Form(request): Form<Request>,
) -> impl IntoResponse {
    match insert_subscriber(&pool, &request.name, &request.email).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(name = "Saving a new subscriber details in the database", skip_all)]
async fn insert_subscriber(
    pool: &Pool<Postgres>,
    name: &str,
    email: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO subscriptions (id, name, email, subscribed_at) VALUES ($1, $2, $3, $4)",
        Uuid::new_v7(Timestamp::now(ContextV7::new())),
        name,
        email,
        Utc::now().naive_utc(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
