use rax::str_parser::rules::{Char, Until};
use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::NmeaUtc;
use crate::macros::readonly_struct;
use crate::nmea_data::{INmeaData, Talker};

readonly_struct!(
    Dhv ,
    "Dhv",
    {navigation_system: Talker},

    {utc_time: Option<chrono::DateTime<chrono::Utc>>},
    {speed3d : Option<f64>},
    {speed_x: Option<f64>},
    {speed_y: Option<f64>},
    {speed_z: Option<f64>},
    {gdspd: Option<f64>}
);
impl INmeaData for Dhv {
    fn new(
        ctx: &mut StrParserContext,
        navigation_system: Talker,
    ) -> miette::Result<Self> {
        let char_comma = Char(&',');
        let until_comma = Until(",");
        let until_star = Until("*");

        let utc_time = ctx
            .skip_strict(&until_comma)?
            .skip_strict(&char_comma)?
            .take(&NmeaUtc());
        let speed3d = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let speed_x = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let speed_y = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let speed_z = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let gdspd = ctx.skip_strict(&char_comma)?.take(&until_star).parse_opt();

        Ok(Dhv {
            navigation_system,
            utc_time,
            speed3d,
            speed_x,
            speed_y,
            speed_z,
            gdspd,
        })
    }
}

#[cfg(test)]
mod test {
    use test_utils::init_log;

    use super::*;
    #[test]
    fn test_new_dhv() -> miette::Result<()> {
        init_log();
        let s = "$GNDHV,021150.000,0.03,0.006,-0.042,-0.026,0.06*65";
        let mut ctx = StrParserContext::new();
        let dhv = Dhv::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", dhv);
        Ok(())
    }
}
