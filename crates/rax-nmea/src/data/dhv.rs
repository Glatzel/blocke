use rax_parser::str_parser::rules::{Char, Until};
use rax_parser::str_parser::{ParseOptExt, StrParserContext};

use crate::NmeaUtc;
use crate::macros::readonly_struct;
use crate::data::{INmeaData, Talker};

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
    fn new(ctx: &mut StrParserContext, navigation_system: Talker) -> miette::Result<Self> {
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

use std::fmt;

impl fmt::Debug for Dhv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("Dhv");
        ds.field("navigation_system", &self.navigation_system);

        if let Some(ref utc_time) = self.utc_time {
            ds.field("utc_time", utc_time);
        }
        if let Some(speed3d) = self.speed3d {
            ds.field("speed3d", &speed3d);
        }
        if let Some(speed_x) = self.speed_x {
            ds.field("speed_x", &speed_x);
        }
        if let Some(speed_y) = self.speed_y {
            ds.field("speed_y", &speed_y);
        }
        if let Some(speed_z) = self.speed_z {
            ds.field("speed_z", &speed_z);
        }
        if let Some(gdspd) = self.gdspd {
            ds.field("gdspd", &gdspd);
        }

        ds.finish()
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
