use std::fmt;

use rax_parser::str_parser::rules::{Char, Until};
use rax_parser::str_parser::{ParseOptExt, StrParserContext};
use serde::{Deserialize, Serialize};

use crate::data::Talker;
use crate::macros::readonly_struct;
use crate::sign::*;
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Satellite {
    id: Option<u16>,
    elevation_degrees: Option<u8>,
    azimuth_degree: Option<u16>,
    snr: Option<u8>,
}
impl fmt::Debug for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("Satellite");

        if let Some(ref id) = self.id {
            ds.field("id", id);
        }
        if let Some(elevation_degrees) = self.elevation_degrees {
            ds.field("elevation_degrees", &elevation_degrees);
        }
        if let Some(azimuth_degree) = self.azimuth_degree {
            ds.field("azimuth_degree", &azimuth_degree);
        }
        if let Some(snr) = self.snr {
            ds.field("snr", &snr);
        }
        ds.finish()
    }
}

readonly_struct!(
    Gsv ,
    "Gsv",
    {talker: Talker},
    {satellites: Vec<Satellite>}
);

impl Gsv {
    pub fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        clerk::trace!("Txt::new: sentence='{}'", ctx.full_str());

        // calculate counts
        let line_count = ctx.full_str().lines().count();
        let satellite_count = ctx
            .skip_strict(&*UNTIL_COMMA)?
            .skip_strict(&*CHAR_COMMA)?
            .skip_strict(&*UNTIL_COMMA)?
            .skip_strict(&*CHAR_COMMA)?
            .skip_strict(&*UNTIL_COMMA)?
            .skip_strict(&*CHAR_COMMA)?
            .take(&*UNTIL_COMMA)
            .parse_opt::<usize>()
            .expect("Can not get the count of satellites.");
        let last_line_satellite_count = satellite_count % line_count;

        let mut satellites = Vec::with_capacity(satellite_count);
        //first n-1 lines
        for _ in 0..line_count - 1 {
            for _ in 0..4 {
                let id = ctx
                    .skip_strict(&*CHAR_COMMA)?
                    .take(&*UNTIL_COMMA)
                    .parse_opt();
                let elevation_degrees = ctx
                    .skip_strict(&*CHAR_COMMA)?
                    .take(&*UNTIL_COMMA)
                    .parse_opt();
                let azimuth_degree = ctx
                    .skip_strict(&*CHAR_COMMA)?
                    .take(&*UNTIL_COMMA)
                    .parse_opt();
                let snr = ctx
                    .skip_strict(&*CHAR_COMMA)?
                    .take(&*UNTIL_COMMA)
                    .parse_opt();
                satellites.push(Satellite {
                    id,
                    elevation_degrees,
                    azimuth_degree,
                    snr,
                });
            }
            ctx.skip(&*UNTIL_COMMA)
                .skip(&*UNTIL_COMMA)
                .skip(&*UNTIL_COMMA);
        }
        //middle line
        for _ in 0..last_line_satellite_count {
            let id = ctx
                .skip_strict(&*CHAR_COMMA)?
                .take(&*UNTIL_COMMA)
                .parse_opt();
            let elevation_degrees = ctx
                .skip_strict(&*CHAR_COMMA)?
                .take(&*UNTIL_COMMA)
                .parse_opt();
            let azimuth_degree = ctx
                .skip_strict(&*CHAR_COMMA)?
                .take(&*UNTIL_COMMA)
                .parse_opt();
            let snr = ctx
                .skip_strict(&*CHAR_COMMA)?
                .take(&*UNTIL_COMMA)
                .parse_opt();
            satellites.push(Satellite {
                id,
                elevation_degrees,
                azimuth_degree,
                snr,
            });
        }
        // last line

        Ok(Self { talker, satellites })
    }
}

impl fmt::Debug for Gsv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("GSV");
        ds.field("talker", &self.talker);
        ds.field("satellites", &self.satellites);

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use test_utils::init_log;

    use super::*;
    #[test]
    fn test_new_gsv() -> miette::Result<()> {
        init_log();
        let s = "$GPGSV,3,1,10,25,68,053,47,21,59,306,49,29,56,161,49,31,36,265,49*79\r\n $GPGSV,3,2,10,12,29,048,49,05,22,123,49,18,13,000,49,01,00,000,49*72\r\n$GPGSV,3,3,10,14,00,000,03,16,00,000,27*7C";
        let mut ctx = StrParserContext::new();
        let gsv = Gsv::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{:?}", gsv);
        Ok(())
    }
}
