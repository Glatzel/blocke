use std::str::FromStr;

use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};

use crate::primitives::NavigationSystem;
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
impl FromStr for GgaQualityIndicator {
    type Err = miette::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Invalid),
            "1" => Ok(Self::GpsFix),
            "2" => Ok(Self::DifferentialGpsFix),
            "3" => Ok(Self::PpsFix),
            "4" => Ok(Self::RealTimeKinematic),
            "5" => Ok(Self::FloatRTK),
            "6" => Ok(Self::DeadReckoning),
            "7" => Ok(Self::ManualInputMode),
            "8" => Ok(Self::SimulationMode),
            other => miette::bail!("Unknown GgaQualityIndicator {}", other),
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct Gga {
    /// Navigation system
    pub navigation_system: NavigationSystem,
    pub is_valid: bool,

    pub utc_time: Option<chrono::DateTime<chrono::Utc>>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub quality: Option<GgaQualityIndicator>,
    pub hdop: Option<u8>,
    pub altitude: Option<f64>,
    pub geoid_separation: Option<f64>,
    pub age_of_differential_gps_data: Option<f64>,
    pub differential_reference_station_id: Option<u16>,
}

impl crate::parser::NmeaParser {
    pub fn new_gga(sentence: &str) -> miette::Result<Gga> {
        let parts: Vec<&str> = sentence.split(',').collect();
        println!("{}", &parts[0][1..2]);
        Ok(Gga {
            navigation_system: Self::get_navigation_system(&sentence)?,
            is_valid: Self::is_valid(sentence),

            utc_time: Self::parse_utc(&parts, 2)?,
            lat: Self::parse_latitude(&parts, 3, 4)?,
            lon: Self::parse_latitude(&parts, 5, 6)?,
            quality: Self::parse_primitive(&parts, 7)?,
            hdop: Self::parse_primitive(&parts, 8)?,
            altitude: Self::parse_primitive(&parts, 9)?,
            geoid_separation: Self::parse_primitive(&parts, 10)?,
            age_of_differential_gps_data: Self::parse_primitive(&parts, 12)?,
            differential_reference_station_id: Self::parse_primitive(&parts, 13)?,
        })
    }
}
#[cfg(test)]
mod test {
    use crate::parser::NmeaParser;
    #[test]
    fn test_new_gga() -> miette::Result<()> {
        let s = "$GPGGA,235316.000,2959.9925,S,12000.0090,E,1,06,1.21,62.77,M,0.00,M,,*7B";
        let _ = NmeaParser::new_gga(s)?;
        Ok(())
    }
}
