use chrono::NaiveDate;
use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::data::{FaaMode, Status, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

readonly_struct!(
    Rmc ,
    "Rmc",
    {talker: Talker},

    {
        utc_time: Option<chrono::DateTime<chrono::Utc>>,
        "UTC time of the position fix"
    },
    {
        status: Option<Status>,
        "Status"
    },
    {
        latitude: Option<f64>,
        "Latitude"
    },
    {
        longitude: Option<f64>,
        "Longitude"
    },
    {
        speed_over_ground: Option<f64>,
        "Speed over ground"
    },
    {
        track_made_good: Option<f64>,
        "Track made good"
    },
    {
        date: Option<NaiveDate>,
        "Date"
    },
    {
        magnetic_variation: Option<f64>,
        "Magnetic variation"
    },
    {
        faa_mode: Option<FaaMode>,
        "FAA mode"
    }
);

impl Rmc {
    pub fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;

        let utc_time = ctx
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .take(&NMEA_UTC);
        let status = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let latitude = ctx.skip_strict(&CHAR_COMMA)?.take(&NMEA_COORD);
        let longitude = ctx.skip_strict(&CHAR_COMMA)?.take(&NMEA_COORD);
        let speed_over_ground = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let track_made_good = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let date = ctx.skip_strict(&CHAR_COMMA)?.take(&NMEA_DATE);
        let magnetic_variation = ctx.skip_strict(&CHAR_COMMA)?.take(&NMEA_DEGREE);
        let faa_mode = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_STAR).parse_opt();
        Ok(Rmc {
            talker,
            utc_time,
            status,
            latitude,
            longitude,
            speed_over_ground,
            track_made_good,
            date,
            magnetic_variation,
            faa_mode,
        })
    }
}

use std::fmt;

impl fmt::Debug for Rmc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("RMC");
        ds.field("talker", &self.talker);

        if let Some(ref utc_time) = self.utc_time {
            ds.field("utc_time", utc_time);
        }
        if let Some(ref status) = self.status {
            ds.field("status", status);
        }
        if let Some(latitude) = self.latitude {
            ds.field("latitude", &latitude);
        }
        if let Some(longitude) = self.longitude {
            ds.field("longitude", &longitude);
        }
        if let Some(speed_over_ground) = self.speed_over_ground {
            ds.field("speed_over_ground", &speed_over_ground);
        }
        if let Some(track_made_good) = self.track_made_good {
            ds.field("track_made_good", &track_made_good);
        }
        if let Some(ref date) = self.date {
            ds.field("date", date);
        }
        if let Some(magnetic_variation) = self.magnetic_variation {
            ds.field("magnetic_variation", &magnetic_variation);
        }
        if let Some(ref faa_mode) = self.faa_mode {
            ds.field("faa_mode", faa_mode);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use clerk::init_log_with_level;
    use clerk::tracing::level_filters::LevelFilter;
    use float_cmp::assert_approx_eq;

    use super::*;
    #[test]
    fn test_new_rmc1() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPRMC,110125,A,5505.337580,N,03858.653666,E,148.8,84.6,310317,8.9,E,D*2E";
        let mut ctx = StrParserContext::new();
        let rmc = Rmc::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", rmc);
        assert_eq!(rmc.talker, Talker::GN);
        assert!(rmc.utc_time.unwrap().to_string().contains("11:01:25"));
        assert_eq!(rmc.status.unwrap(), Status::Valid);
        assert_approx_eq!(f64, rmc.latitude.unwrap(), 55.088959666666675);
        assert_approx_eq!(f64, rmc.longitude.unwrap(), 38.9775611);
        assert_approx_eq!(f64, rmc.speed_over_ground.unwrap(), 148.8);
        assert_approx_eq!(f64, rmc.track_made_good.unwrap(), 84.6);
        assert_eq!(rmc.date.unwrap().to_string(), "2017-03-31");
        assert_approx_eq!(f64, rmc.magnetic_variation.unwrap(), 8.9);
        assert_eq!(rmc.faa_mode.unwrap(), FaaMode::Differential);
        Ok(())
    }
    #[test]
    fn test_new_rmc2() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPRMC,,V,,,,,,,,,,N*53";
        let mut ctx = StrParserContext::new();
        let rmc = Rmc::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", rmc);
        assert_eq!(rmc.talker, Talker::GN);
        assert!(rmc.utc_time.is_none());
        assert_eq!(rmc.status, Some(Status::Invalid));
        assert!(rmc.latitude.is_none());
        assert!(rmc.longitude.is_none());
        assert!(rmc.speed_over_ground.is_none());
        assert!(rmc.track_made_good.is_none());
        assert!(rmc.date.is_none());
        assert!(rmc.magnetic_variation.is_none());
        assert_eq!(rmc.faa_mode, Some(FaaMode::NotValid));
        Ok(())
    }
}
