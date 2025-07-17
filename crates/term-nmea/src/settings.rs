use std::path::PathBuf;
use std::sync::RwLock;
use std::{fs, io};

use miette::IntoDiagnostic;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::cli::CliArgs;

pub static SETTINGS: OnceCell<RwLock<Settings>> = OnceCell::new();

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
        // Initialize the global SETTINGS once with RwLock
        SETTINGS
            .set(RwLock::new(settings))
            .map_err(|_| miette::miette!("SETTINGS already initialized"))?;
        Self::save()?;
        Ok(())
    }
    pub fn path() -> PathBuf {
        // Locate config file next to the binary

        std::env::current_exe()
            .map(|exe| exe.with_file_name("term-nmea.toml"))
            .unwrap_or_else(|e| {
                clerk::warn!("Cannot determine executable path: {e}. Using defaults.");
                PathBuf::from("term-nmea.toml")
            })
    }

    pub fn save() -> miette::Result<()> {
        // Get the global SETTINGS static
        let settings_lock = SETTINGS
            .get()
            .ok_or_else(|| miette::miette!("SETTINGS not initialized"))?;

        // Acquire a read lock to access current settings
        let settings = settings_lock
            .read()
            .map_err(|e| miette::miette!("Failed to lock SETTINGS for reading: {}", e))?;

        // Save settings to the file
        let toml_str = toml::to_string_pretty(&*settings)
            .map_err(|e| io::Error::other(format!("TOML serialize error: {e}")))
            .into_diagnostic()?;
        fs::write(Self::path(), toml_str).into_diagnostic()
    }
}
impl Settings {
    /// Get a read-only reference to the global Settings.
    pub fn reader() -> std::sync::RwLockReadGuard<'static, Settings> {
        SETTINGS
            .get()
            .expect("SETTINGS not initialized")
            .read()
            .expect("RwLock poisoned")
    }
    /// Get a writer reference to the global Settings.
    pub fn _writer() -> std::sync::RwLockWriteGuard<'static, Settings> {
        SETTINGS
            .get()
            .expect("SETTINGS not initialized")
            .write()
            .expect("RwLock poisoned")
    }

    /// Get a clone of the current port.
    pub fn port() -> String { Self::reader().port.clone() }

    /// Optional: add more helpers
    pub fn baud_rate() -> u32 { Self::reader().baud_rate }

    pub fn capacity() -> usize { Self::reader().capacity }
}
#[cfg(test)]
mod tests {
    use clap_verbosity_flag::Verbosity;
    use tempfile::tempdir;

    use super::*;
    use crate::cli::CliArgs;

    #[test]
    fn test_default_init_and_save() {
        let config_path = Settings::path();

        let cli = CliArgs {
            port: None,
            baud_rate: None,
            capacity: None,
            verbose: Verbosity::new(0, 0),
        };

        Settings::init(&cli).expect("Should initialize default settings");

        let settings = Settings::reader();
        assert_eq!(settings.port, "COM1");
        assert_eq!(settings.baud_rate, 9600);
        assert_eq!(settings.capacity, 1000);

        // Check that config file was written
        assert!(config_path.exists());
        let contents = std::fs::read_to_string(config_path).unwrap();
        assert!(contents.contains("port = \"COM1\""));
    }

    #[test]
    fn test_init_with_cli_override() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let cli = CliArgs {
            port: Some("COM9".to_string()),
            baud_rate: Some(115_200),
            capacity: Some(2048),
            verbose: Verbosity::new(0, 0),
        };

        Settings::init(&cli).expect("Init with CLI override");
        let s = Settings::reader();

        assert_eq!(s.port, "COM9");
        assert_eq!(s.baud_rate, 115_200);
        assert_eq!(s.capacity, 2048);
    }
    #[test]
    fn test_malformed_config_fallbacks_to_default() {
        let config_path = Settings::path();

        // Write malformed TOML
        std::fs::write(&config_path, "bad = [this is: not toml").unwrap();

        let cli = CliArgs {
            port: None,
            baud_rate: None,
            capacity: None,
            verbose: Verbosity::new(0, 0),
        };

        Settings::init(&cli).expect("Should fallback to default on malformed");

        let s = Settings::reader();
        assert_eq!(s.port, "COM1");
    }
}
