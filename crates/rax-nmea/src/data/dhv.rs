use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::data::{INmeaData, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

readonly_struct!(
    Dhv ,
    "Dhv",
    {talker: Talker},

    {
        utc_time: Option<chrono::DateTime<chrono::Utc>>,
        "UTC time of the DHV fix associated with this sentence."
    },
    {
        speed3d : Option<f64>,
        "3D speed (meters/second)"
    },
    {
        speed_x: Option<f64>,
        "Speed in X direction (meters/second)"
    },
    {
        speed_y: Option<f64>,
        "Speed in Y direction (meters/second)"
    },
    {
        speed_z: Option<f64>,
        "Speed in Z direction (meters/second)"
    },
    {
        gdspd: Option<f64>,
        "Ground speed (meters/second)"
    }
);
impl INmeaData for Dhv {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;
        let utc_time = ctx
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .take(&NMEA_UTC);
        let speed3d = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let speed_x = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let speed_y = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let speed_z = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let gdspd = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_STAR).parse_opt();

        Ok(Dhv {
            talker,
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
        let mut ds = f.debug_struct("DHV");
        ds.field("talker", &self.talker);

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

    use clerk::init_log_with_level;
    use clerk::tracing::level_filters::LevelFilter;

    use super::*;
    #[test]
    fn test_new_dhv() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GNDHV,021150.000,0.03,0.006,-0.042,-0.026,0.06*65";
        let mut ctx = StrParserContext::new();
        let dhv = Dhv::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", dhv);
        assert_eq!(dhv.talker, Talker::GN);
        assert!(dhv.utc_time.unwrap().to_string().contains("02:11:50"));
        assert_eq!(dhv.speed3d.unwrap(), 0.03);
        assert_eq!(dhv.speed_x.unwrap(), 0.006);
        assert_eq!(dhv.speed_y.unwrap(), -0.042);
        assert_eq!(dhv.speed_z.unwrap(), -0.026);
        assert_eq!(dhv.gdspd.unwrap(), 0.06);
        Ok(())
    }
}
