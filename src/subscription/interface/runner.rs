use std::error::Error;

use tokio::net::TcpListener;

use crate::subscription::domain::service::CommandExecutor;
use crate::subscription::interface::router::get_router;
use crate::subscription::interface::router::Container;

pub async fn run(
    listener: TcpListener,
    command_executor: CommandExecutor,
) -> Result<(), impl Error> {
    let container = Container::new(command_executor);
    let app = get_router(container).await;

    axum::serve(listener, app).await
}
