use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};

use lru::LruCache;
use miette::IntoDiagnostic;

use crate::INmeaData;

pub struct NmeaContext<T>
where
    T: INmeaData,
{
    cache: Arc<Mutex<LruCache<crate::NmeaType, T>>>,
    output: Option<Mutex<File>>,
}
impl<T> NmeaContext<T>
where
    T: INmeaData,
{
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
    pub fn push(&self, sentense: &str) -> &Self { self }
}
