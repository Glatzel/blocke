use miette::IntoDiagnostic;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use crate::{primitives::NavigationSystem, traits::INmeaData};
#[derive(Debug, Clone, Copy, TryFromPrimitive, strum::Display, Serialize, Deserialize)]
#[repr(u8)]
pub enum GgaQualityIndicator {
    Invalid = 0,
    GpsFix = 1,
    DifferentialGpsFix = 2,
    PpsFix = 3,
    RealTimeKinematic = 4,
    FloatRTK = 5,
    DeadReckoning = 6,
    ManualInputMode = 7,
    SimulationMode = 8,
}
#[derive(Serialize, Deserialize)]
pub struct Gga {
    /// Navigation system
    pub source: NavigationSystem,
    pub utc_time: chrono::DateTime<chrono::Utc>,
    pub lat: f64,
    pub lon: f64,
    pub quality: GgaQualityIndicator,
    pub hdop: u8,
    pub altitude: f64,
    pub geoid_separation: f64,
    pub age_of_differential_gps_data: f64,
    pub differential_reference_station_id: u16,
}

// impl INmeaData for Gga {
//     fn parse_sentence(sentence: &str) -> miette::Result<Self> {
//         let parts: Vec<&str> = sentence.split(',').collect();
//         if parts.len() < 10 {
//             miette::bail!("Not enough fields for GGA");
//         }
//         Ok(Gga {
//             source: NavigationSystem::from_str(&parts[0][0..2]).into_diagnostic()?,
//             utc_time: chrono::Utc::now(),
//             lat: parts[2].parse::<f64>().unwrap_or(0.0),
//             lon: parts[4].parse::<f64>().unwrap_or(0.0),
//             quality: GgaQualityIndicator::Invalid,
//             hdop: parts[8].parse::<u8>().unwrap_or(0),
//             altitude: parts[9].parse::<f64>().unwrap_or(0.0),
//             geoid_separation: 0.0,
//             age_of_differential_gps_data: 0.0,
//             differential_reference_station_id: 0,
//         })
//     }
// }
