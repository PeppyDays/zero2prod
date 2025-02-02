use std::sync::Arc;

use axum::extract::FromRef;
use axum::routing::get;
use axum::routing::post;
use axum::Router;

use crate::subscriber::domain::service::CommandExecutor;
use crate::subscriber::interface::controllers;

#[derive(Clone)]
pub struct Container {
    command_executor: Arc<dyn CommandExecutor>,
}

impl Container {
    pub fn new(command_executor: impl CommandExecutor) -> Self {
        Self {
            command_executor: Arc::new(command_executor),
        }
    }
}

impl FromRef<Container> for Arc<dyn CommandExecutor> {
    fn from_ref(container: &Container) -> Self {
        container.command_executor.clone()
    }
}

pub async fn get_router(container: Container) -> Router {
    Router::new()
        .route(
            "/subscriptions",
            post(controllers::post_subscriptions::control),
        )
        .route(
            "/subscriptions/confirm",
            get(controllers::get_subscriptions_confirm::control),
        )
        .with_state(container)
}
