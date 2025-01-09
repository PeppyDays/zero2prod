use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_bunyan_formatter::JsonStorageLayer;
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;

pub fn initialise_tracing() {
    // Set up default values
    let name = "zero2prod".to_string();
    let env_filter = "info".to_string();
    let sink = std::io::stdout;

    // Create a filter for the tracing layer
    // RUST_LOG is the environment variable that controls the verbosity of the logs
    // if it is not set, we default to "info"
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    // Create a formatting layer, currently using Bunyan
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    // Create a subscriber with the filter and the formatting layer
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");

    // Set our subscriber as the global default to process spans
    set_global_default(subscriber).expect("Failed to set subscriber");
}
