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

pub async fn subscribe(
    State(pool): State<Pool<Postgres>>,
    Form(request): Form<Request>,
) -> impl IntoResponse {
    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, name, email, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v7(Timestamp::now(ContextV7::new())),
        request.name,
        request.email,
        Utc::now().naive_utc(),
    )
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
