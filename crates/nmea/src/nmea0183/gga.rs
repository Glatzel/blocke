use std::str::FromStr;

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
#[derive(Serialize, Deserialize, Debug)]
pub struct Gga {
    /// Navigation system
    pub navigation_system: NavigationSystem,
    pub is_valid: bool,

    pub utc_time: Option<chrono::DateTime<chrono::Utc>>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub quality: Option<GgaQualityIndicator>,
    pub satellite_count: Option<u8>,
    pub hdop: Option<f64>,
    pub altitude: Option<f64>,
    pub geoid_separation: Option<f64>,
    pub age_of_differential_gps_data: Option<f64>,
    pub differential_reference_station_id: Option<u16>,
}

impl crate::parser::NmeaParser {
    pub fn new_gga(sentence: &str) -> miette::Result<Gga> {
        let parts: Vec<&str> = sentence.split(',').collect();
        Ok(Gga {
            navigation_system: Self::get_navigation_system(&sentence)?,
            is_valid: Self::is_valid(sentence),

            utc_time: Self::parse_utc(&parts, 1)?,
            lat: Self::parse_latitude(&parts, 2, 3)?,
            lon: Self::parse_longitude(&parts, 4, 5)?,
            quality: Self::parse_primitive(&parts, 6)?,
            satellite_count: Self::parse_primitive(&parts, 7)?,
            hdop: Self::parse_primitive(&parts, 8)?,
            altitude: Self::parse_primitive(&parts, 9)?,
            geoid_separation: Self::parse_primitive(&parts, 11)?,
            age_of_differential_gps_data: Self::parse_primitive(&parts, 13)?,
            differential_reference_station_id: Self::parse_primitive(&parts, 14)?,
        })
    }
}
#[cfg(test)]
mod test {
    use test_utils::init_log;

    use crate::parser::NmeaParser;
    #[test]
    fn test_new_gga() -> miette::Result<()> {
        init_log();
        let s = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
        for (i, v) in s.split(",").enumerate() {
            println!("{i}:{v}");
        }
        let gga = NmeaParser::new_gga(s)?;
        println!("{:?}", gga);
        assert!(gga.is_valid);
        Ok(())
    }
}
