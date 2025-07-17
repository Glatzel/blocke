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
    port: String,
    baud_rate: u32,
    capacity: usize,
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
    pub fn _writter() -> std::sync::RwLockWriteGuard<'static, Settings> {
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
