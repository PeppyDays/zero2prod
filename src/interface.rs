use std::error::Error;

use axum::Router;
use tokio::net::TcpListener;

use crate::subscriber;

pub async fn run(
    listener: TcpListener,
    subscriber_command_executor: impl subscriber::domain::service::CommandExecutor,
) -> Result<(), impl Error> {
    let subscriber_container =
        subscriber::interface::router::Container::new(subscriber_command_executor);
    let subscriber_router = subscriber::interface::router::get_router(subscriber_container).await;

    let app = Router::new().merge(subscriber_router);

    axum::serve(listener, app).await
}
