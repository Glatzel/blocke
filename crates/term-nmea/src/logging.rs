use std::path::PathBuf;
use std::sync::OnceLock;

use chrono::Local;
use clap_verbosity_flag::{Verbosity, VerbosityFilter};
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
            VerbosityFilter::Error => LevelFilter::ERROR,
            VerbosityFilter::Warn => LevelFilter::WARN,
            VerbosityFilter::Info => LevelFilter::INFO,
            VerbosityFilter::Debug => LevelFilter::DEBUG,
            VerbosityFilter::Trace => LevelFilter::TRACE,
            VerbosityFilter::Off => return,
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
#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_generate_log_filename_format() {
        let path = generate_log_filename();
        let filename = path.file_name().unwrap().to_string_lossy();

        assert!(filename.starts_with("log-term-nmea-"));
        assert!(filename.ends_with(".log"));

        // Check that directory is correct
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));
        assert!(path.starts_with(exe_dir));
    }

    #[test]
    fn test_logging_initialization_creates_file() {
        // Clean or create "log" folder before test
        let log_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join("log")))
            .unwrap_or_else(|| PathBuf::from("log"));

        let _ = fs::create_dir_all(&log_dir);

        // Set verbosity to trigger logging
        let verbosity = Verbosity::new(3, 0); // -vvv (Trace)

        // Call init()
        init(&verbosity);

        // Check that at least one log file was created
        let entries: Vec<_> = fs::read_dir(log_dir)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|e| {
                e.file_name()
                    .to_string_lossy()
                    .starts_with("log-term-nmea-")
            })
            .collect();

        assert!(
            !entries.is_empty(),
            "Expected at least one log file created"
        );
    }

    #[test]
    fn test_logging_only_initializes_once() {
        let verbosity1 = Verbosity::new(2, 0); // -vv (Debug)
        let verbosity2 = Verbosity::new(0, 0); // no verbosity (Off)

        init(&verbosity1); // should initialize with Debug
        init(&verbosity2); // should be ignored

        // Not easy to verify log level directly (without a custom mock), but we ensure
        // no panic
        assert!(LOG_INIT.get().is_some());
    }
}
