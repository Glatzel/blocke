use std::fmt;
use std::str::FromStr;

use rax_parser::str_parser::{ParseOptExt, StrParserContext};
use serde::{Deserialize, Serialize};

use crate::data::{INmeaData, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

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
    {talker: Talker},

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
    fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        clerk::trace!("Gga::new: sentence='{}'", ctx.full_str());

        ctx.global(&NMEA_VALIDATE)?;

        clerk::debug!("Parsing utc_time...");
        let utc_time = ctx
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .take(&NMEA_UTC);
        clerk::debug!("utc_time: {:?}", utc_time);

        clerk::debug!("Parsing lat...");
        let lat = ctx.skip_strict(&CHAR_COMMA)?.take(&NMEA_COORD);
        clerk::debug!("lat: {:?}", lat);

        clerk::debug!("Parsing lon...");
        let lon = ctx.skip_strict(&CHAR_COMMA)?.take(&NMEA_COORD);
        clerk::debug!("lon: {:?}", lon);

        clerk::debug!("Parsing quality...");
        let quality = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        clerk::debug!("quality: {:?}", quality);

        clerk::debug!("Parsing satellite_count...");
        let satellite_count = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        clerk::debug!("satellite_count: {:?}", satellite_count);

        clerk::debug!("Parsing hdop...");
        let hdop = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        clerk::debug!("hdop: {:?}", hdop);

        clerk::debug!("Parsing altitude...");
        let altitude = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        clerk::debug!("altitude: {:?}", altitude);

        clerk::debug!("Skipping char_comma and char_m for altitude units...");
        ctx.skip_strict(&CHAR_COMMA)?.skip(&CHAR_M);

        clerk::debug!("Parsing geoid_separation...");
        let geoid_separation = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        clerk::debug!("geoid_separation: {:?}", geoid_separation);

        clerk::debug!("Skipping char_comma and char_m for geoid units...");
        ctx.skip_strict(&CHAR_COMMA)?.skip(&CHAR_M);

        clerk::debug!("Parsing age_of_differential_gps_data...");
        let age_of_differential_gps_data =
            ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        clerk::debug!(
            "age_of_differential_gps_data: {:?}",
            age_of_differential_gps_data
        );

        clerk::debug!("Parsing differential_reference_station_id...");
        let differential_reference_station_id =
            ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_STAR).parse_opt();
        clerk::debug!(
            "differential_reference_station_id: {:?}",
            differential_reference_station_id
        );

        Ok(Gga {
            talker,
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

impl fmt::Debug for Gga {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("GGA");
        ds.field("talker", &self.talker);

        if let Some(ref utc_time) = self.utc_time {
            ds.field("utc_time", utc_time);
        }
        if let Some(lat) = self.lat {
            ds.field("lat", &lat);
        }
        if let Some(lon) = self.lon {
            ds.field("lon", &lon);
        }
        if let Some(ref quality) = self.quality {
            ds.field("quality", quality);
        }
        if let Some(satellite_count) = self.satellite_count {
            ds.field("satellite_count", &satellite_count);
        }
        if let Some(hdop) = self.hdop {
            ds.field("hdop", &hdop);
        }
        if let Some(altitude) = self.altitude {
            ds.field("altitude", &altitude);
        }
        if let Some(geoid_separation) = self.geoid_separation {
            ds.field("geoid_separation", &geoid_separation);
        }
        if let Some(age_of_differential_gps_data) = self.age_of_differential_gps_data {
            ds.field(
                "age_of_differential_gps_data",
                &age_of_differential_gps_data,
            );
        }
        if let Some(differential_reference_station_id) = self.differential_reference_station_id {
            ds.field(
                "differential_reference_station_id",
                &differential_reference_station_id,
            );
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use test_utils::init_log;

    use super::*;

    #[test]
    fn test_new_gga1() -> miette::Result<()> {
        init_log();
        let s = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
        let mut ctx = StrParserContext::new();
        let gga = Gga::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", gga);

        Ok(())
    }
    #[test]
    fn test_new_gga2() -> miette::Result<()> {
        init_log();
        let s = "$GNGGA,130301.000,,,,,0,00,25.5,,,,,,*7A";
        let mut ctx = StrParserContext::new();
        let gga = Gga::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", gga);

        Ok(())
    }
}
