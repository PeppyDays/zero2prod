use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080))
        .await
        .expect("Failed to bind a port");

    let app = Router::new().route("/{name}", get(greet));

    axum::serve(listener, app).await
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    format!("hello, {name}!")
}
