use rax_parser::str_parser::{ParseOptExt, StrParserContext};

use crate::data::{INmeaData, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;
readonly_struct!(
    Gst ,
    "Gst",
    {talker: Talker},

    {utc_time: Option<chrono::DateTime<chrono::Utc>>},
    {rms  : Option<f64>},
    {std_dev_semi_major: Option<f64>},
    {std_dev_semi_minor: Option<f64>},
    {std_dev_semi_latitude: Option<f64>},
    {std_dev_semi_longitude: Option<f64>},
    {std_dev_semi_altitude: Option<f64>}
);
impl INmeaData for Gst {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;

        let utc_time = ctx
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .take(&NMEA_UTC);
        let rms = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let std_dev_semi_major = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let std_dev_semi_minor = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let std_dev_semi_latitude = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let std_dev_semi_longitude = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let std_dev_semi_altitude = ctx.skip_strict(&UNTIL_STAR)?.take(&UNTIL_COMMA).parse_opt();

        Ok(Gst {
            talker,
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

use std::fmt;

impl fmt::Debug for Gst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("GST");
        ds.field("talker", &self.talker);

        if let Some(ref utc_time) = self.utc_time {
            ds.field("utc_time", utc_time);
        }
        if let Some(rms) = self.rms {
            ds.field("rms", &rms);
        }
        if let Some(std_dev_semi_major) = self.std_dev_semi_major {
            ds.field("std_dev_semi_major", &std_dev_semi_major);
        }
        if let Some(std_dev_semi_minor) = self.std_dev_semi_minor {
            ds.field("std_dev_semi_minor", &std_dev_semi_minor);
        }
        if let Some(std_dev_semi_latitude) = self.std_dev_semi_latitude {
            ds.field("std_dev_semi_latitude", &std_dev_semi_latitude);
        }
        if let Some(std_dev_semi_longitude) = self.std_dev_semi_longitude {
            ds.field("std_dev_semi_longitude", &std_dev_semi_longitude);
        }
        if let Some(std_dev_semi_altitude) = self.std_dev_semi_altitude {
            ds.field("std_dev_semi_altitude", &std_dev_semi_altitude);
        }

        ds.finish()
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
