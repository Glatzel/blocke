use rax_parser::str_parser::rules::{Char, Until};
use rax_parser::str_parser::{ParseOptExt, StrParserContext};

use crate::NmeaUtc;
use crate::macros::readonly_struct;
use crate::nmea_data::{INmeaData, Talker};

readonly_struct!(
    Gst ,
    "Gst",
    {navigation_system: Talker},

    {utc_time: Option<chrono::DateTime<chrono::Utc>>},
    {rms  : Option<f64>},
    {std_dev_semi_major: Option<f64>},
    {std_dev_semi_minor: Option<f64>},
    {std_dev_semi_latitude: Option<f64>},
    {std_dev_semi_longitude: Option<f64>},
    {std_dev_semi_altitude: Option<f64>}
);
impl INmeaData for Gst {
    fn new(ctx: &mut StrParserContext, navigation_system: Talker) -> miette::Result<Self> {
        let char_comma = Char(&',');
        let until_comma = Until(",");
        let until_star = Until("*");

        let utc_time = ctx
            .skip_strict(&until_comma)?
            .skip_strict(&char_comma)?
            .take(&NmeaUtc());
        let rms = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let std_dev_semi_major = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let std_dev_semi_minor = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let std_dev_semi_latitude = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let std_dev_semi_longitude = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let std_dev_semi_altitude = ctx.skip_strict(&until_star)?.take(&until_comma).parse_opt();

        Ok(Gst {
            navigation_system,
            utc_time,
            rms,
            std_dev_semi_major,
            std_dev_semi_minor,
            std_dev_semi_latitude,
            std_dev_semi_longitude,
            std_dev_semi_altitude,
        })
    }
}

#[cfg(test)]
mod test {
    use test_utils::init_log;

    use super::*;
    #[test]
    fn test_new_gst() -> miette::Result<()> {
        init_log();
        let s = "$GPGST,182141.000,15.5,15.3,7.2,21.8,0.9,0.5,0.8*54";
        let mut ctx = StrParserContext::new();
        let vtg = Gst::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", vtg);

        Ok(())
    }
}
