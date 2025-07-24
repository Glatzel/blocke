use std::path::PathBuf;
use std::sync::OnceLock;
use std::{fs, io};

use clap_verbosity_flag::VerbosityFilter;
use clerk::LogLevel;
use directories::ProjectDirs;
use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};

use crate::cli::CliArgs;
pub static SETTINGS: OnceLock<Settings> = OnceLock::new();
mod tab_coord;
pub use tab_coord::TabCoordSettings;
#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub port: String,
    pub baud_rate: u32,
    pub capacity: usize,
    pub verbose: LogLevel,

    pub tab_coord: TabCoordSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            port: "COM1".into(), // pick sensible defaults for your platform
            baud_rate: 9_600,
            capacity: 1000,
            verbose: LogLevel::ERROR,

            tab_coord: TabCoordSettings {
                custom_cs: String::default(),
            },
        }
    }
}
impl Settings {
    /// Initialise the config: try to read `term‑nmea.toml` located
    /// in the same directory as the executable.
    /// Falls back to `Config::default()` if the file is missing
    /// or malformed.
    pub fn init(cli: &CliArgs) -> miette::Result<()> {
        let path = Self::path();
        // Read from file or fallback
        let mut settings = match fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_else(|e| {
                clerk::warn!("Malformed config file: {e}. Using defaults.");
                Self::default()
            }),
            Err(e) => {
                clerk::warn!("Failed to read config: {e}. Using defaults.");
                Self::default()
            }
        };

        // Override with CLI args
        if let Some(ref port) = cli.port {
            settings.port = port.clone();
        }
        if let Some(baud) = cli.baud_rate {
            settings.baud_rate = baud;
        }
        if let Some(cap) = cli.capacity {
            settings.capacity = cap;
        }
        settings.verbose = match cli.verbose.filter() {
            VerbosityFilter::Error => LogLevel::ERROR,
            VerbosityFilter::Warn => LogLevel::WARN,
            VerbosityFilter::Info => LogLevel::INFO,
            VerbosityFilter::Debug => LogLevel::DEBUG,
            VerbosityFilter::Trace => LogLevel::TRACE,
            VerbosityFilter::Off => LogLevel::OFF,
        };

        // Initialize the global SETTINGS once with RwLock
        SETTINGS
            .set(settings)
            .map_err(|_| miette::miette!("SETTINGS already initialized"))?;
        Self::save()?;
        Ok(())
    }
    pub fn path() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "pyxis-trail") {
            proj_dirs.config_dir().join("pyxis-trail.toml")
        } else {
            clerk::warn!("Cannot determine config directory. Using local file.");
            PathBuf::from("pyxis-trail.toml")
        }
    }

    pub fn save() -> miette::Result<()> {
        let settings = SETTINGS
            .get()
            .ok_or_else(|| miette::miette!("SETTINGS not initialized"))?;

        let toml_str = toml::to_string_pretty(settings)
            .map_err(|e| io::Error::other(format!("TOML serialize error: {e}")))
            .into_diagnostic()?;

        let path = Self::path();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| io::Error::other(format!("Failed to create config directory: {e}")))
                .into_diagnostic()?;
        }

        fs::write(path, toml_str)
            .map_err(|e| io::Error::other(format!("Failed to write config: {e}")))
            .into_diagnostic()
    }
}
