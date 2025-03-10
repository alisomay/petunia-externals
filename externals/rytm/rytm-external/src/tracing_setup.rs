use parking_lot::Mutex;
use std::sync::Arc;
use tracing::Level;
use tracing_core::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, reload, EnvFilter, Layer};

pub type ReloadHandle = reload::Handle<EnvFilter, tracing_subscriber::Registry>;

pub struct LoggingState {
    pub reload_handle: ReloadHandle,
    pub active_level: Mutex<tracing::Level>,
}

pub fn get_default_env_filter() -> EnvFilter {
    EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_env_var("RYTM_LOG")
        .from_env_lossy()
}

pub fn setup_logging() -> (
    Arc<dyn tracing::Subscriber + Send + Sync + 'static>,
    Arc<LoggingState>,
) {
    let (env_filter, reload_handle) =
        reload::Layer::<EnvFilter, tracing_subscriber::Registry>::new(get_default_env_filter());

    let logging_state = Arc::new(LoggingState {
        reload_handle,
        active_level: Mutex::new(Level::INFO),
    });

    let env_filter = env_filter.boxed();
    let fmt_layer = tracing_subscriber::fmt::layer().pretty().boxed();

    let layers = env_filter.and_then(fmt_layer).boxed();
    let registry = tracing_subscriber::registry().with(layers);

    (Arc::new(registry), logging_state)
}
