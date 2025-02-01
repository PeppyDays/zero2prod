use std::error::Error;

use zero2prod::assembly;
use zero2prod::configuration;
use zero2prod::interface;
use zero2prod::subscriber;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> Result<(), impl Error> {
    // Set up telemetry
    telemetry::initialise_tracing();

    // Read configuration
    let env: configuration::Environment = std::env::var("ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to determine environment");
    let mut configuration =
        configuration::get_configuration(env).expect("Failed to read configuration");

    // Set up listener for running this application
    let listener = assembly::get_application_listener(&configuration.application).await;

    // Assemble subscriber aggregate's external dependencies
    let subscriber_database_pool =
        assembly::get_database_pool(&configuration.subscriber.database).await;
    let subscriber_repository =
        assembly::assemble_subscriber_repository(subscriber_database_pool.clone());
    let subscription_token_repository =
        assembly::assemble_subscription_token_repository(subscriber_database_pool.clone());

    let _ = assembly::assemble_subscription_email_server(&mut configuration.subscriber.email).await;
    let subscription_email_client =
        assembly::assemble_subscription_email_client(&configuration.subscriber.email);

    // Assemble subscriber aggregate's command executor
    let subscriber_command_executor = subscriber::domain::service::new_command_executor(
        subscriber_repository,
        subscription_token_repository,
        subscription_email_client,
    );

    // Run this application
    interface::run(listener, subscriber_command_executor).await
}
