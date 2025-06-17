use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};

use lru::LruCache;
use miette::IntoDiagnostic;

pub struct NmeaContext {
    cache: Arc<Mutex<LruCache<&'static str, crate::nmea_data::GenericNmeaData>>>,
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
        let data = crate::nmea_data::GenericNmeaData::try_from(sentense)?;
        let key: &'static str = match data {
            crate::nmea_data::GenericNmeaData::DHV(_) => "dhv",
            crate::nmea_data::GenericNmeaData::GGA(_) => "gga",
            crate::nmea_data::GenericNmeaData::GLL(_) => "gll",
            crate::nmea_data::GenericNmeaData::GSA(_) => "gsa",
            crate::nmea_data::GenericNmeaData::VTG(_) => "vtg",
            crate::nmea_data::GenericNmeaData::ZDA(_) => "zda",
            crate::nmea_data::GenericNmeaData::Other(_) => "other",
        };

        {
            let mut cache = self.cache.lock().unwrap();
            cache.put(key, data);
        }

        Ok(self)
    }
}
