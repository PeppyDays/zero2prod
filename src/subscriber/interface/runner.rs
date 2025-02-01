use std::error::Error;

use tokio::net::TcpListener;

use crate::subscriber::domain::service::CommandExecutor as SubscriberCommandExecutor;
use crate::subscriber::interface::router::get_router;
use crate::subscriber::interface::router::Container;

pub async fn run(
    listener: TcpListener,
    subscriber_command_executor: impl SubscriberCommandExecutor,
) -> Result<(), impl Error> {
    let container = Container::new(subscriber_command_executor);
    let app = get_router(container).await;

    axum::serve(listener, app).await
}
