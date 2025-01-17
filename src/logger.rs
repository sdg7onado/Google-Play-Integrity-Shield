use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};

pub fn init() {
    let subscriber = SubscriberBuilder::default()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber for Proxima Integrity API.");
}
