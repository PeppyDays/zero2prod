use std::io::Error;
use std::net::{Ipv4Addr, SocketAddrV4};

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

pub async fn check_health() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn run() -> Result<(), Error> {
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080))
        .await
        .expect("Failed to bind a port");

    let app = Router::new().route("/health_check", get(check_health));

    axum::serve(listener, app).await
}
