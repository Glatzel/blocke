use rax_parser::str_parser::rules::{Char, Until};
use rax_parser::str_parser::{ParseOptExt, StrParserContext};

use crate::data::{FaaMode, INmeaData, Talker};
use crate::macros::readonly_struct;

readonly_struct!(
    Vtg ,
    "Vtg",
    {navigation_system: Talker},

    {course_over_ground_true: Option<f64>},
    {course_over_ground_magnetic : Option<f64>},
    {speed_over_ground_knots: Option<f64>},
    {speed_over_ground_kph: Option<f64>},
    {mode: Option<FaaMode>}
);
impl INmeaData for Vtg {
    fn new(ctx: &mut StrParserContext, navigation_system: Talker) -> miette::Result<Self> {
        let char_comma = Char(&',');
        let until_comma = Until(",");
        let until_star = Until("*");

        let course_over_ground_true = ctx
            .skip_strict(&until_comma)?
            .take(&until_comma)
            .parse_opt();
        let course_over_ground_magnetic =
            ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let speed_over_ground_knots = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let speed_over_ground_kph = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let mode = ctx.skip_strict(&char_comma)?.take(&until_star).parse_opt();

        Ok(Vtg {
            navigation_system,
            course_over_ground_true,
            course_over_ground_magnetic,
            speed_over_ground_knots,
            speed_over_ground_kph,
            mode,
        })
    }
}

use std::fmt;

impl fmt::Debug for Vtg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("Vtg");
        ds.field("navigation_system", &self.navigation_system);

        if let Some(course_over_ground_true) = self.course_over_ground_true {
            ds.field("course_over_ground_true", &course_over_ground_true);
        }
        if let Some(course_over_ground_magnetic) = self.course_over_ground_magnetic {
            ds.field("course_over_ground_magnetic", &course_over_ground_magnetic);
        }
        if let Some(speed_over_ground_knots) = self.speed_over_ground_knots {
            ds.field("speed_over_ground_knots", &speed_over_ground_knots);
        }
        if let Some(speed_over_ground_kph) = self.speed_over_ground_kph {
            ds.field("speed_over_ground_kph", &speed_over_ground_kph);
        }
        if let Some(ref mode) = self.mode {
            ds.field("mode", mode);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use test_utils::init_log;

    use super::*;
    #[test]
    fn test_new_vtg() -> miette::Result<()> {
        init_log();
        let s = "$GPVTG,220.86,T,,M,2.550,N,4.724,K,A*34";
        let mut ctx = StrParserContext::new();
        let vtg = Vtg::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", vtg);

        Ok(())
    }
}
