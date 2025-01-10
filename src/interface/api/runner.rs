use std::error::Error;

use tokio::net::TcpListener;

use crate::domain::repository::SubscriptionRepository;
use crate::interface::api::router::get_router;
use crate::interface::api::router::Container;

pub async fn run(
    listener: TcpListener,
    repository: impl SubscriptionRepository,
) -> Result<(), impl Error> {
    let container = Container::new(repository);
    let app = get_router(container).await;

    axum::serve(listener, app).await
}
