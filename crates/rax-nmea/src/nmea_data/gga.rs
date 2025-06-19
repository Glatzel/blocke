use std::str::FromStr;

use rax::str_parser::rules::{Char, Until};
use rax::str_parser::{ParseOptExt, StrParserContext};
use serde::{Deserialize, Serialize};

use crate::NmeaUtc;
use crate::macros::readonly_struct;
use crate::nmea_data::NavigationSystem;

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
impl Gga {
    fn new(sentence: &'static str, navigation_system: NavigationSystem) -> miette::Result<Gga> {
        let char_comma = Char(&',');
        let char_m = Char(&'M');
        let until_comma = Until(",");
        let until_star = Until("*");

        let mut ctx = StrParserContext::new(sentence);

        let utc_time = ctx
            .skip_strict(&until_comma)?
            .skip_strict(&char_comma)?
            .take(&NmeaUtc());
        let lat = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let lon = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let quality = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let satellite_count = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let hdop = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let altitude = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let geoid_separation = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let age_of_differential_gps_data =
            ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let differential_reference_station_id =
            ctx.skip_strict(&char_comma)?.take(&until_star).parse_opt();

        Ok(Gga {
            navigation_system,
            utc_time,
            lat,
            lon,
            quality,
            satellite_count,
            hdop,
            altitude,
            geoid_separation,
            age_of_differential_gps_data,
            differential_reference_station_id,
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

        let gga = Gga::new(s, NavigationSystem::GN)?;
        println!("{:?}", gga);

        Ok(())
    }
}
