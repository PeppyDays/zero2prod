use std::sync::Arc;

use axum::extract::FromRef;
use axum::routing::get;
use axum::routing::post;
use axum::Router;

use crate::subscriber::domain::service::CommandExecutor;
use crate::subscriber::domain::service::PluralQueryReader;
use crate::subscriber::domain::service::SingularQueryReader;
use crate::subscriber::interface::controllers;

#[derive(Clone)]
pub struct Container {
    command_executor: Arc<dyn CommandExecutor>,
    singular_query_reader: Arc<dyn SingularQueryReader>,
    plural_query_reader: Arc<dyn PluralQueryReader>,
}

impl Container {
    pub fn new(
        command_executor: impl CommandExecutor,
        singular_query_reader: impl SingularQueryReader,
        plural_query_reader: impl PluralQueryReader,
    ) -> Self {
        Self {
            command_executor: Arc::new(command_executor),
            singular_query_reader: Arc::new(singular_query_reader),
            plural_query_reader: Arc::new(plural_query_reader),
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
        .route("/subscribers", get(controllers::get_subscribers::control))
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
