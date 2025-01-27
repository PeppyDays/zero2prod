use std::error::Error;

use tokio::net::TcpListener;

use crate::domain::subscriber::service::command::interface::ExecuteCommand as ExecuteSubscriberCommand;
use crate::interface::router::get_router;
use crate::interface::router::Container;

pub async fn run(
    listener: TcpListener,
    execute_subscriber_command: ExecuteSubscriberCommand,
) -> Result<(), impl Error> {
    let container = Container::new(execute_subscriber_command);
    let app = get_router(container).await;

    axum::serve(listener, app).await
}
