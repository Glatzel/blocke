use rax_parser::str_parser::{ParseOptExt, StrParserContext};

use crate::data::{FaaMode, INmeaData, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

readonly_struct!(
    Vtg ,
    "Vtg",
    {talker: Talker},

    {
        course_over_ground_true: Option<f64>,
        "Course over ground (true)"
    },
    {
        course_over_ground_magnetic: Option<f64>,
        "Course over ground (magnetic)"
    },
    {
        speed_over_ground_knots: Option<f64>,
        "Speed over ground (knots)"
    },
    {
        speed_over_ground_kph: Option<f64>,
        "Speed over ground (kph)"
    },
    {
        mode: Option<FaaMode>,
        "Mode"
    }
);
impl INmeaData for Vtg {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;

        let course_over_ground_true = ctx
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .take(&UNTIL_COMMA)
            .parse_opt();
        ctx.skip_strict(&CHAR_COMMA)?.skip(&CHAR_T);

        let course_over_ground_magnetic =
            ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        ctx.skip_strict(&CHAR_COMMA)?.skip(&CHAR_M);

        let speed_over_ground_knots = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        ctx.skip_strict(&CHAR_COMMA)?.skip(&CHAR_N);

        let speed_over_ground_kph = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        ctx.skip_strict(&CHAR_COMMA)?.skip(&CHAR_K);

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
            ds.field(
                "course_over_ground_true",
                &format!("{} Degrees", course_over_ground_true),
            );
        }
        if let Some(course_over_ground_magnetic) = self.course_over_ground_magnetic {
            ds.field(
                "course_over_ground_magnetic",
                &format!("{} Degrees", course_over_ground_magnetic),
            );
        }
        if let Some(speed_over_ground_knots) = self.speed_over_ground_knots {
            ds.field(
                "speed_over_ground_knots",
                &format!("{} Knots", speed_over_ground_knots),
            );
        }
        if let Some(speed_over_ground_kph) = self.speed_over_ground_kph {
            ds.field(
                "speed_over_ground_kph",
                &format!("{} Kph", speed_over_ground_kph),
            );
        }
        if let Some(ref mode) = self.mode {
            ds.field("mode", mode);
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
    fn test_new_vtg() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPVTG,83.7,T,83.7,M,146.3,N,271.0,K,D*22";
        let mut ctx = StrParserContext::new();
        let vtg = Vtg::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", vtg);
        assert_eq!(vtg.talker, Talker::GN);
        assert_approx_eq!(f64, vtg.course_over_ground_true.unwrap(), 83.7);
        assert_approx_eq!(f64, vtg.course_over_ground_magnetic.unwrap(), 83.7);
        assert_approx_eq!(f64, vtg.speed_over_ground_knots.unwrap(), 146.3);
        assert_approx_eq!(f64, vtg.speed_over_ground_kph.unwrap(), 271.0);
        assert_eq!(vtg.mode.unwrap(), FaaMode::Differential);
        Ok(())
    }
}
