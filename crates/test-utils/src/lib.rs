use std::sync::LazyLock;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

static INIT_LOGGING: LazyLock<bool> = LazyLock::new(|| {
    tracing_subscriber::registry()
        .with(clerk::terminal_layer(LevelFilter::INFO, true))
        .init();
    true
});
pub fn init_log() { let _ = &*INIT_LOGGING; }
