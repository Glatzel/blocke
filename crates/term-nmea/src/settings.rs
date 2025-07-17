use std::path::PathBuf;
use std::{fs, io};

use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};

use crate::cli::CliArgs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub port: String,
    pub baud_rate: u32,
    pub capacity: usize,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            port: "COM1".into(), // pick sensible defaults for your platform
            baud_rate: 9_600,
            capacity: 1000,
        }
    }
}
impl Settings {
    /// Initialise the config: try to read `term‑nmea.toml` located
    /// in the same directory as the executable.
    /// Falls back to `Config::default()` if the file is missing
    /// or malformed.
    pub fn init(cli: &CliArgs) -> miette::Result<Self> {
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
        settings.save_to(&path);
        Ok(settings)
    }
    pub fn path() -> PathBuf {
        // Locate config file next to the binary
        let path = std::env::current_exe()
            .map(|exe| exe.with_file_name("term-nmea.toml"))
            .unwrap_or_else(|e| {
                clerk::warn!("Cannot determine executable path: {e}. Using defaults.");
                PathBuf::from("term-nmea.toml")
            });
        path
    }
    pub fn save_to(&self, path: &PathBuf) -> miette::Result<()> {
        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| io::Error::other(format!("TOML serialize error: {e}")))
            .into_diagnostic()?;
        fs::write(path, toml_str).into_diagnostic()
    }
}
