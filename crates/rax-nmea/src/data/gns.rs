use std::fmt::Debug;
use std::str::FromStr;

use rax::str_parser::{ParseOptExt, StrParserContext};
use serde::{Deserialize, Serialize};

use crate::data::{FaaMode, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum NavigationStatus {
    Safe,
    Caution,
    Unsafe,
    Invalid,
}
impl FromStr for NavigationStatus {
    type Err = miette::Report;

    fn from_str(s: &str) -> miette::Result<Self> {
        match s {
            "S" => Ok(Self::Safe),
            "C" => Ok(Self::Caution),
            "U" => Ok(Self::Unsafe),
            "V" => Ok(Self::Invalid),
            _ => miette::bail!("Unknown NavigationStatus: {}", s),
        }
    }
}
readonly_struct!(
    Gns,
    "Gns",
    {talker: Talker},

    {
        utc_time:Option<chrono::DateTime<chrono::Utc>>,
        "UTC time of the position fix"
    },
    {
        latitude: Option<f64>,
        "Latitude, ddmm.mmmm, where dd is degrees and mm.mmmm is minutes. Positive values indicate North, negative values indicate South."
    },
    {
        longitude: Option<f64>,
        "Longitude, dddmm.mmmm, where ddd is degrees and mm.mmmm is minutes. Positive values indicate East, negative values indicate West."
    },
    {
        mode: [FaaMode;2],
        "FAA mode"
    },
    {
        satellites :Option<u8>,
        "Number of satellites in use"
    },
    {
        hdop:Option<f64>,
        "Horizontal dilution of precision"
    },
    {
        altitude:Option<f64>,
        "Altitude"
    },
    {
        goeidal_separation:Option<f64>,
        "Geoidal separation"
    },
    {
        differential_data_age:Option<f64>,
        "Differential data age"
    },
    {
        differential_reference_station_id:Option<u16>,
        "Differential reference station ID"
    },
    {
        navigational_status:Option<NavigationStatus>,
        "Navigational status"
    }
);

impl Gns {
    pub fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
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

        clerk::debug!("Parsing mode...");
        let mode_str = ctx
            .skip_strict(&CHAR_COMMA)?
            .take(&UNTIL_COMMA)
            .expect("Mode string should not be empty.");
        let mode = [
            FaaMode::from_str(mode_str.get(0..1).unwrap())?,
            FaaMode::from_str(mode_str.get(1..2).unwrap())?,
        ];
        clerk::debug!("mode: {:?}", mode);

        clerk::debug!("Parsing satellites...");
        let satellites = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        clerk::debug!("satellites: {:?}", satellites);

        clerk::debug!("Parsing hdop...");
        let hdop = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        clerk::debug!("hdop: {:?}", hdop);

        clerk::debug!("Parsing altitude...");
        let altitude = ctx.skip(&CHAR_COMMA).take(&UNTIL_COMMA_OR_STAR).parse_opt();
        clerk::debug!("altitude: {:?}", altitude);

        clerk::debug!("Parsing goeidal_separation...");
        let goeidal_separation = ctx.skip(&CHAR_COMMA).take(&UNTIL_COMMA_OR_STAR).parse_opt();
        clerk::debug!("goeidal_separation: {:?}", goeidal_separation);

        clerk::debug!("Parsing differential_data_age...");
        let differential_data_age = ctx.skip(&CHAR_COMMA).take(&UNTIL_COMMA_OR_STAR).parse_opt();
        clerk::debug!("differential_data_age: {:?}", differential_data_age);

        clerk::debug!("Parsing differential_reference_station_id...");

        let differential_reference_station_id =
            ctx.skip(&CHAR_COMMA).take(&UNTIL_COMMA_OR_STAR).parse_opt();

        clerk::debug!(
            "differential_reference_station_id: {:?}",
            differential_reference_station_id
        );

        clerk::debug!("Parsing navigational_status...");
        let navigational_status = ctx.skip(&CHAR_COMMA).take(&UNTIL_STAR).parse_opt();
        clerk::debug!("navigational_status: {:?}", navigational_status);

        Ok(Gns {
            talker,
            utc_time,
            latitude: lat,
            longitude: lon,
            mode,
            satellites,
            hdop,
            altitude,
            goeidal_separation,
            differential_data_age,
            differential_reference_station_id,
            navigational_status,
        })
    }
}
impl Debug for Gns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("GNS");
        ds.field("talker", &self.talker);
        if let Some(ref utc_time) = self.utc_time {
            ds.field("utc_time", utc_time);
        }
        if let Some(lat) = self.latitude {
            ds.field("latitude", &lat);
        }
        if let Some(lon) = self.longitude {
            ds.field("longitude", &lon);
        }
        ds.field("mode", &self.mode);
        if let Some(satellites) = self.satellites {
            ds.field("satellites", &satellites);
        }
        if let Some(hdop) = self.hdop {
            ds.field("hdop", &hdop);
        }
        if let Some(altitude) = self.altitude {
            ds.field("altitude", &altitude);
        }
        if let Some(goeidal_separation) = self.goeidal_separation {
            ds.field("goeidal_separation", &goeidal_separation);
        }
        if let Some(differential_data_age) = self.differential_data_age {
            ds.field("differential_data_age", &differential_data_age);
        }
        if let Some(differential_reference_station_id) = self.differential_reference_station_id {
            ds.field(
                "differential_reference_station_id",
                &differential_reference_station_id,
            );
        }
        if let Some(navigational_status) = &self.navigational_status {
            ds.field("navigational_status", navigational_status);
        }
        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use clerk::{LevelFilter, init_log_with_level};
    use float_cmp::assert_approx_eq;

    use super::*;
    use crate::data::{FaaMode, Talker};

    #[test]
    fn test_gns_parsing1() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPGNS,112257.00,3844.24011,N,00908.43828,W,AN,03,10.5,,*57";
        let mut ctx = StrParserContext::new();
        let gns = Gns::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", gns);
        assert_eq!(gns.talker, Talker::GN);
        assert!(gns.utc_time.unwrap().to_string().contains("11:22:57"));
        assert_eq!(gns.latitude.unwrap(), 38.73733516666667);
        assert_eq!(gns.longitude.unwrap(), -9.140638);
        assert_eq!(gns.mode, [FaaMode::Autonomous, FaaMode::NotValid]);
        assert_eq!(gns.satellites.unwrap(), 3);
        assert_eq!(gns.hdop.unwrap(), 10.5);
        assert!(gns.altitude.is_none());
        assert!(gns.goeidal_separation.is_none());
        assert!(gns.differential_data_age.is_none());
        assert!(gns.differential_reference_station_id.is_none());

        Ok(())
    }
    #[test]
    fn test_gns_parsing2() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GNGNS,181604.00,,,,,NN,00,99.99,,,,*59";
        let mut ctx = StrParserContext::new();
        let gns = Gns::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", gns);
        assert_eq!(gns.talker, Talker::GN);
        assert!(gns.utc_time.unwrap().to_string().contains("18:16:04"));
        assert!(gns.latitude.is_none());
        assert!(gns.longitude.is_none());
        assert_eq!(gns.mode, [FaaMode::NotValid, FaaMode::NotValid]);
        assert_eq!(gns.satellites.unwrap(), 0);
        assert_approx_eq!(f64, gns.hdop.unwrap(), 99.99);
        assert!(gns.altitude.is_none());
        assert!(gns.goeidal_separation.is_none());
        assert!(gns.differential_data_age.is_none());
        assert!(gns.differential_reference_station_id.is_none());
        assert!(gns.navigational_status.is_none());
        Ok(())
    }
}
