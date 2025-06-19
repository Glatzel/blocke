use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::INmeaData;
use crate::nmea_data::NavigationSystem;
use crate::rules::{readonly_struct, *};

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
    "Gga",
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
impl INmeaData for Gga {
    fn parse_sentence(sentence: &str, navigation_system: NavigationSystem) -> miette::Result<Gga> {
        let parts: Vec<&str> = get_sentence_parts(sentence);
        Ok(Gga {
            navigation_system,
            is_valid: is_valid(sentence),
            utc_time: parse_utc(&parts, 1)?,
            lat: parse_latitude(&parts, 2, 3)?,
            lon: parse_longitude(&parts, 4, 5)?,
            quality: parse_primitive(&parts, 6)?,
            satellite_count: parse_primitive(&parts, 7)?,
            hdop: parse_primitive(&parts, 8)?,
            altitude: parse_primitive(&parts, 9)?,
            geoid_separation: parse_primitive(&parts, 11)?,
            age_of_differential_gps_data: parse_primitive(&parts, 13)?,
            differential_reference_station_id: parse_primitive(&parts, 14)?,
        })
    }
}

#[cfg(test)]
mod test {
    use test_utils::init_log;

    use super::*;

    #[test]
    fn test_new_gga() -> miette::Result<()> {
        init_log();
        let s = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
        for (i, v) in get_sentence_parts(s).iter().enumerate() {
            println!("{i}:{v}");
        }
        let gga = Gga::parse_sentence(s, NavigationSystem::GN)?;
        println!("{:?}", gga);
        assert!(gga.is_valid);
        Ok(())
    }
}
