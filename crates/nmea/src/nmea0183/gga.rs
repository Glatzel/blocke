use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::NavigationSystem;
use crate::utils::readonly_struct;

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

readonly_struct!(
    Gga ,
    "",
    {navigation_system: NavigationSystem},
    {is_valid: bool},

    {utc_time: Option<chrono::DateTime<chrono::Utc>>},
    {lat: Option<f64>},
    {lon: Option<f64>},
    {quality: Option<GgaQualityIndicator>},
    {satellite_count: Option<u8>},
    {hdop: Option<f64>},
    {altitude: Option<f64>},
    {geoid_separation: Option<f64>},
    {age_of_differential_gps_data: Option<f64>},
    {differential_reference_station_id: Option<u16>}
);
impl crate::parser::NmeaParser {
    pub fn new_gga(sentence: &str) -> miette::Result<Gga> {
        let parts: Vec<&str> = Self::get_sentense_parts(sentence);
        Ok(Gga::new(
            Self::get_navigation_system(&sentence)?,
            Self::is_valid(sentence),
            Self::parse_utc(&parts, 1)?,
            Self::parse_latitude(&parts, 2, 3)?,
            Self::parse_longitude(&parts, 4, 5)?,
            Self::parse_primitive(&parts, 6)?,
            Self::parse_primitive(&parts, 7)?,
            Self::parse_primitive(&parts, 8)?,
            Self::parse_primitive(&parts, 9)?,
            Self::parse_primitive(&parts, 11)?,
            Self::parse_primitive(&parts, 13)?,
            Self::parse_primitive(&parts, 14)?,
        ))
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
        for (i, v) in NmeaParser::get_sentense_parts(s).iter().enumerate() {
            println!("{i}:{v}");
        }
        let gga = NmeaParser::new_gga(s)?;
        println!("{:?}", gga);
        assert!(gga.is_valid);
        Ok(())
    }
}
