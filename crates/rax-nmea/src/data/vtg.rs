use rax_parser::str_parser::{ParseOptExt, StrParserContext};

use crate::data::{FaaMode, INmeaData, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

readonly_struct!(
    Vtg ,
    "Vtg",
    {talker: Talker},

    {course_over_ground_true: Option<f64>},
    {course_over_ground_magnetic : Option<f64>},
    {speed_over_ground_knots: Option<f64>},
    {speed_over_ground_kph: Option<f64>},
    {mode: Option<FaaMode>}
);
impl INmeaData for Vtg {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;

        let course_over_ground_true = ctx
            .skip_strict(&UNTIL_COMMA)?
            .take(&UNTIL_COMMA)
            .parse_opt();
        let course_over_ground_magnetic =
            ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let speed_over_ground_knots = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let speed_over_ground_kph = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let mode = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_STAR).parse_opt();

        Ok(Vtg {
            talker,
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
        let mut ds = f.debug_struct("VTG");
        ds.field("talker", &self.talker);

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
    use clerk::tracing::level_filters::LevelFilter;
    use clerk::init_log_with_level;
    use super::*;
    #[test]
    fn test_new_vtg() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPVTG,220.86,T,,M,2.550,N,4.724,K,A*34";
        let mut ctx = StrParserContext::new();
        let vtg = Vtg::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", vtg);

        Ok(())
    }
}
