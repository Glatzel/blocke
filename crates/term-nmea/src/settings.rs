use std::path::PathBuf;
use std::{fs, io};

use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};

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
    pub fn init() -> miette::Result<Self> {
        // Where is the executable?
        let path = match std::env::current_exe().map(|exe| exe.with_file_name("term-nmea.toml")) {
            Ok(p) => p,
            Err(e) => {
                clerk::warn!("Cannot determine executable path: {e}. Using defaults.");
                return Ok(Self::default());
            }
        };

        // Try to read the file; missing file is not an error.
        match fs::read_to_string(&path) {
            Ok(toml_str) => toml::from_str(&toml_str).into_diagnostic(),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                // File not found: create with default values
                let default = Self::default();
                if let Err(e) = default.save_to(&path) {
                    clerk::warn!("Failed to write default config: {e}");
                } else {
                    clerk::info!("Created default config at {}", path.display());
                }
                Ok(default)
            }
            Err(e) => {
                clerk::warn!("Failed to read config file: {e}. Using defaults.");
                Ok(Self::default())
            }
        }
    }
    pub fn save_to(&self, path: &PathBuf) -> miette::Result<()> {
        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| io::Error::other(format!("TOML serialize error: {e}")))
            .into_diagnostic()?;
        fs::write(path, toml_str).into_diagnostic()
    }
}
