use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use lru::LruCache;
use miette::IntoDiagnostic;

use crate::nmea_data::{Dhv, GenericNmeaData, Gga, Gll, Gsa, NmeaDataType, Vtg, Zda};
use crate::{INmeaData, NavigationSystem};

pub struct NmeaContext {
    cache: Arc<Mutex<LruCache<NmeaDataType, crate::nmea_data::GenericNmeaData>>>,
    output: Option<Mutex<File>>,
}
impl NmeaContext {
    /// Create a new NMEA context with given capacity.
    pub fn new(capacity: usize, output_path: Option<&Path>) -> miette::Result<Self> {
        let file = match output_path {
            Some(path) => Some(Mutex::new(File::create(path).into_diagnostic()?)),
            None => None,
        };
        let cache = LruCache::new(std::num::NonZeroUsize::new(capacity).unwrap());
        Ok(Self {
            cache: Arc::new(Mutex::new(cache)),
            output: file,
        })
    }
    pub fn push(&self, sentense: &str) -> miette::Result<&Self> {
        let key = NmeaDataType::from_str(sentense)?;
        let navigation_system = NavigationSystem::from_str(&sentense[1..3]).into_diagnostic()?;

        let data = match key {
            NmeaDataType::DHV => {
                GenericNmeaData::DHV(Dhv::parse_sentense(sentense, navigation_system)?)
            }
            NmeaDataType::GGA => {
                GenericNmeaData::GGA(Gga::parse_sentense(sentense, navigation_system)?)
            }
            NmeaDataType::GLL => {
                GenericNmeaData::GLL(Gll::parse_sentense(sentense, navigation_system)?)
            }
            NmeaDataType::GSA => {
                GenericNmeaData::GSA(Gsa::parse_sentense(sentense, navigation_system)?)
            }
            NmeaDataType::VTG => {
                GenericNmeaData::VTG(Vtg::parse_sentense(sentense, navigation_system)?)
            }
            NmeaDataType::ZDA => {
                GenericNmeaData::ZDA(Zda::parse_sentense(sentense, navigation_system)?)
            }
            NmeaDataType::Other => GenericNmeaData::Other(sentense.to_string()),
        };

        let mut cache = self.cache.lock().unwrap();
        cache.put(key, data);

        if let Some(file_mutex) = &self.output {
            if let Ok(mut file) = file_mutex.lock() {
                let _ = writeln!(file, "{}", sentense);
            }
        }
        Ok(self)
    }
    /// Get a cached NMEA data value by type.
    pub fn get(&self, key: NmeaDataType) -> Option<GenericNmeaData> {
        let mut cache = self.cache.lock().unwrap();
        cache.get(&key).cloned()
    }

    /// Check if a sentence type exists in the cache.
    pub fn contains(&self, nmea_type: NmeaDataType) -> bool {
        let cache = self.cache.lock().unwrap();
        cache.contains(&nmea_type)
    }

    /// Get number of items currently cached.
    pub fn len(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.len()
    }

    /// Check if the cache is empty.
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
