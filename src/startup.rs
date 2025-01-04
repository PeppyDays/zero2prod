use std::error::Error;

use axum::routing::get;
use axum::routing::post;
use axum::Router;
use tokio::net::TcpListener;

use crate::routes;

pub async fn run(listener: TcpListener) -> Result<(), impl Error> {
    let app = Router::new()
        .route("/subscriptions", post(routes::subscriptions::subscribe))
        .route("/healthz", get(routes::health_check::check_health));

    axum::serve(listener, app).await
}
