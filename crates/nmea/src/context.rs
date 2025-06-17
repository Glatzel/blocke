use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use lru::LruCache;
use miette::IntoDiagnostic;

use crate::INmeaData;
use crate::nmea_data::{
    Dhv, GenericNmeaData, Gga, Gll, Gsa, NavigationSystem, NmeaDataType, Vtg, Zda,
};

pub struct NmeaContext {
    cache:
        Arc<Mutex<LruCache<(NavigationSystem, NmeaDataType), crate::nmea_data::GenericNmeaData>>>,
    output: Option<Mutex<File>>,
    skip_navigation_system: Vec<NavigationSystem>,
    skip_nmea_data_type: Vec<NmeaDataType>,
}
impl NmeaContext {
    /// Create a new NMEA context with given capacity.
    pub fn new(
        capacity: usize,
        output_path: Option<&Path>,
        skip_navigation_system: Option<Vec<NavigationSystem>>,
        skip_nmea_data_type: Option<Vec<NmeaDataType>>,
    ) -> miette::Result<Self> {
        let file = match output_path {
            Some(path) => Some(Mutex::new(File::create(path).into_diagnostic()?)),
            None => None,
        };
        let cache = LruCache::new(std::num::NonZeroUsize::new(capacity).unwrap());
        Ok(Self {
            cache: Arc::new(Mutex::new(cache)),
            output: file,
            skip_navigation_system: skip_navigation_system.unwrap_or_default(),
            skip_nmea_data_type: skip_nmea_data_type.unwrap_or_default(),
        })
    }
    pub fn push(&self, sentense: &str) -> miette::Result<&Self> {
        let data_type = NmeaDataType::from_str(sentense)?;
        let navigation_system = NavigationSystem::from_str(sentense)?;

        //skip
        if self.skip_navigation_system.contains(&navigation_system)
            && self.skip_nmea_data_type.contains(&data_type)
        {
            clerk::info!(
                "Skip,data_type: {}, navigation_system: {}",
                data_type,
                navigation_system
            );
            return Ok(self);
        }

        let data = match &data_type {
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
            NmeaDataType::Other(s) => GenericNmeaData::Other(s.to_string()),
        };

        let mut cache = self.cache.lock().unwrap();
        cache.put((navigation_system, data_type), data);

        if let Some(file_mutex) = &self.output {
            if let Ok(mut file) = file_mutex.lock() {
                let _ = writeln!(file, "{}", sentense);
            }
        }
        Ok(self)
    }
    /// Get a cached NMEA data value by type.
    pub fn get(&self, key: (NavigationSystem, NmeaDataType)) -> Option<GenericNmeaData> {
        let mut cache = self.cache.lock().unwrap();
        cache.get(&key).cloned()
    }

    /// Check if a sentence type exists in the cache.
    pub fn contains(&self, key: (NavigationSystem, NmeaDataType)) -> bool {
        let cache = self.cache.lock().unwrap();
        cache.contains(&key)
    }

    /// Get number of items currently cached.
    pub fn len(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.len()
    }

    /// Check if the cache is empty.
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
