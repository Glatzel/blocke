use std::path::PathBuf;
use std::sync::OnceLock;

use chrono::Local;
use clap_verbosity_flag::Verbosity;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry;
use tracing_subscriber::util::SubscriberInitExt;

// Global guard to ensure init runs once
static LOG_INIT: OnceLock<()> = OnceLock::new();

pub fn init(verbosity: &Verbosity) {
    LOG_INIT.get_or_init(|| {
        // Determine log level
        let log_level = verbosity.filter();
        let tracing_level = match log_level {
            clap_verbosity_flag::VerbosityFilter::Error => LevelFilter::ERROR,
            clap_verbosity_flag::VerbosityFilter::Warn => LevelFilter::WARN,
            clap_verbosity_flag::VerbosityFilter::Info => LevelFilter::INFO,
            clap_verbosity_flag::VerbosityFilter::Debug => LevelFilter::DEBUG,
            clap_verbosity_flag::VerbosityFilter::Trace => LevelFilter::TRACE,
            clap_verbosity_flag::VerbosityFilter::Off => return,
        };

        // Generate log file path with datetime
        let log_file_path = generate_log_filename();

        // Create your custom file layer (assumed here as `clerk::file_layer`)
        let file_layer = clerk::file_layer(tracing_level, log_file_path, true);

        // Register once
        registry().with(file_layer).init();
    });
}
fn generate_log_filename() -> PathBuf {
    let now = Local::now();
    let filename = format!("log/log-term-nmea-{}.log", now.format("%Y-%m-%d-%H-%M-%S"));

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    exe_dir.join(filename)
}
