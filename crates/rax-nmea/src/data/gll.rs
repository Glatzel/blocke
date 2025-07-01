use rax_parser::str_parser::rules::{Char, Until};
use rax_parser::str_parser::{ParseOptExt, StrParserContext};

use crate::macros::readonly_struct;
use crate::data::{FaaMode, INmeaData, Status, Talker};
use crate::{NmeaCoord, NmeaUtc};

readonly_struct!(
    Gll ,
    "Gll",
    {navigation_system: Talker},

    {lat: Option<f64>},
    {lon: Option<f64>},
    {utc_time: Option<chrono::DateTime<chrono::Utc>>},
    {data_valid: Option<Status>},
    {faa_mode: Option<FaaMode>}
);
impl INmeaData for Gll {
    fn new(ctx: &mut StrParserContext, navigation_system: Talker) -> miette::Result<Self> {
        clerk::trace!("Gga::new: sentence='{}'", ctx.full_str());

        let char_comma = Char(&',');
        let until_comma = Until(",");
        let until_star = Until("*");

        clerk::debug!("Parsing lat...");
        let lat = ctx
            .skip_strict(&until_comma)?
            .skip_strict(&char_comma)?
            .take(&NmeaCoord());
        clerk::debug!("lat: {:?}", lat);

        clerk::debug!("Parsing lon...");
        let lon = ctx.skip_strict(&char_comma)?.take(&NmeaCoord());
        clerk::debug!("lon: {:?}", lon);

        clerk::debug!("Parsing utc_time...");
        let utc_time = ctx.skip_strict(&char_comma)?.take(&NmeaUtc());
        clerk::debug!("utc_time: {:?}", utc_time);

        let data_valid = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();

        let faa_mode = ctx.skip_strict(&char_comma)?.take(&until_star).parse_opt();

        Ok(Gll {
            navigation_system,
            lat,
            lon,
            utc_time,
            data_valid,
            faa_mode,
        })
    }
}

#[cfg(test)]
mod test {
    use test_utils::init_log;

    use super::*;
    #[test]
    fn test_new_ggl() -> miette::Result<()> {
        init_log();
        let s = "$GPGLL,2959.9925,S,12000.0090,E,235316.000,A,A*4E";
        let mut ctx = StrParserContext::new();
        let gll = Gll::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", gll);
        Ok(())
    }
}
