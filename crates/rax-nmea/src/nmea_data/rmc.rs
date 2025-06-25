use chrono::NaiveDate;
use rax_parser::str_parser::rules::{Char, Until};
use rax_parser::str_parser::{ParseOptExt, StrParserContext};

use crate::macros::readonly_struct;
use crate::nmea_data::{FaaMode, Status, Talker};
use crate::{NmeaCoord, NmeaDate, NmeaUtc};

readonly_struct!(
    Rmc ,
    "Rmc",
    {navigation_system: Talker},

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
    pub fn new(ctx: &mut StrParserContext, navigation_system: Talker) -> miette::Result<Self> {
        let char_comma = Char(&',');
        let until_comma = Until(",");
        let until_star = Until("*");

        let utc_time = ctx
            .skip_strict(&until_comma)?
            .skip_strict(&char_comma)?
            .take(&NmeaUtc());
        let status = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let latitude = ctx.skip_strict(&char_comma)?.take(&NmeaCoord());
        let longitude = ctx.skip_strict(&char_comma)?.take(&NmeaCoord());
        let speed_over_ground = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let track_made_good = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let date = ctx.skip_strict(&char_comma)?.take(&NmeaDate());
        let magnetic_variation = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let faa_mode = ctx.skip_strict(&char_comma)?.take(&until_star).parse_opt();
        Ok(Rmc {
            navigation_system,
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
