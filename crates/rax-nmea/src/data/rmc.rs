use chrono::NaiveDate;
use rax_parser::str_parser::{ParseOptExt, StrParserContext};

use crate::data::{FaaMode, Status, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;


readonly_struct!(
    Rmc ,
    "Rmc",
    {talker: Talker},

    {utc_time: Option<chrono::DateTime<chrono::Utc>>},
    {status : Option<Status>},
    {latitude: Option<f64>},
    {longitude: Option<f64>},
    {speed_over_ground: Option<f64>},
    {track_made_good: Option<f64>},
    {date:Option<NaiveDate>},
    {magnetic_variation:Option<f64>},
    {faa_mode: Option<FaaMode>}
);

impl Rmc {
    pub fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        let utc_time = ctx
            .skip_strict(&*UNTIL_COMMA)?
            .skip_strict(&*CHAR_COMMA)?
            .take(&*NMEA_UTC);
        let status = ctx.skip_strict(&*CHAR_COMMA)?.take(&*UNTIL_COMMA).parse_opt();
        let latitude = ctx.skip_strict(&*CHAR_COMMA)?.take(&*NMEA_COORD);
        let longitude = ctx.skip_strict(&*CHAR_COMMA)?.take(&*NMEA_COORD);
        let speed_over_ground = ctx.skip_strict(&*CHAR_COMMA)?.take(&*UNTIL_COMMA).parse_opt();
        let track_made_good = ctx.skip_strict(&*CHAR_COMMA)?.take(&*UNTIL_COMMA).parse_opt();
        let date = ctx.skip_strict(&*CHAR_COMMA)?.take(&*NMEA_DATE);
        let magnetic_variation = ctx.skip_strict(&*CHAR_COMMA)?.take(&*UNTIL_COMMA).parse_opt();
        let faa_mode = ctx.skip_strict(&*CHAR_COMMA)?.take(&*UNTIL_STAR).parse_opt();
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
    use test_utils::init_log;

    use super::*;
    #[test]
    fn test_new_rmc() -> miette::Result<()> {
        init_log();
        let s = "$GPRMC,235316.000,A,2959.9925,S,12000.0090,E,0.009,75.020,020711,,,A*45";
        let mut ctx = StrParserContext::new();
        let zda = Rmc::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", zda);
        Ok(())
    }
}
