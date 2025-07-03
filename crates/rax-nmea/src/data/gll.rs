use rax_parser::str_parser::{ParseOptExt, StrParserContext};

use crate::data::{FaaMode, INmeaData, Status, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

readonly_struct!(
    Gll ,
    "Gll",
    {talker: Talker},

    {lat: Option<f64>},
    {lon: Option<f64>},
    {utc_time: Option<chrono::DateTime<chrono::Utc>>},
    {data_valid: Option<Status>},
    {faa_mode: Option<FaaMode>}
);
impl INmeaData for Gll {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        clerk::trace!("Gga::new: sentence='{}'", ctx.full_str());

        ctx.global(&NMEA_VALIDATE)?;

        clerk::debug!("Parsing lat...");
        let lat = ctx
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .take(&NMEA_COORD);
        clerk::debug!("lat: {:?}", lat);

        clerk::debug!("Parsing lon...");
        let lon = ctx.skip_strict(&CHAR_COMMA)?.take(&NMEA_COORD);
        clerk::debug!("lon: {:?}", lon);

        clerk::debug!("Parsing utc_time...");
        let utc_time = ctx.skip_strict(&CHAR_COMMA)?.take(&NMEA_UTC);
        clerk::debug!("utc_time: {:?}", utc_time);

        let data_valid = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();

        let faa_mode = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_STAR).parse_opt();

        Ok(Gll {
            talker,
            lat,
            lon,
            utc_time,
            data_valid,
            faa_mode,
        })
    }
}

use std::fmt;

impl fmt::Debug for Gll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("GLL");
        ds.field("talker", &self.talker);

        if let Some(lat) = self.lat {
            ds.field("lat", &lat);
        }
        if let Some(lon) = self.lon {
            ds.field("lon", &lon);
        }
        if let Some(ref utc_time) = self.utc_time {
            ds.field("utc_time", utc_time);
        }
        if let Some(ref data_valid) = self.data_valid {
            ds.field("data_valid", data_valid);
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
    fn test_new_ggl() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPGLL,2959.9925,S,12000.0090,E,235316.000,A,A*4E";
        let mut ctx = StrParserContext::new();
        let gll = Gll::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", gll);
        assert_eq!(gll.talker, Talker::GN);
        assert_approx_eq!(f64, gll.lat.unwrap(), -29.999874999999996);
        assert_approx_eq!(f64, gll.lon.unwrap(), 120.00015);
        assert!(gll.utc_time.unwrap().to_string().contains("23:53:16"));
        assert_eq!(gll.data_valid.unwrap(), Status::Valid);
        assert_eq!(gll.faa_mode.unwrap(), FaaMode::Autonomous);
        Ok(())
    }
}
