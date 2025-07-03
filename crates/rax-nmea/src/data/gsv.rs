use std::fmt;

use rax_parser::str_parser::{IStrGlobalRule, ParseOptExt, StrParserContext};
use serde::{Deserialize, Serialize};

use crate::data::Talker;
use crate::macros::readonly_struct;
use crate::rules::*;
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
        for l in ctx.full_str().lines() {
            NMEA_VALIDATE.apply(l)?;
        }

        // calculate counts
        let line_count = ctx.full_str().lines().count();
        clerk::trace!("Gsv::new: line_count={line_count}");
        let satellite_count = ctx
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .take(&UNTIL_COMMA)
            .parse_opt::<usize>()
            .expect("Can not get the count of satellites.");
        clerk::trace!("Gsv::new: satellite_count={satellite_count}");
        let last_line_satellite_count = satellite_count % 4;
        let last_line_satellite_count = if last_line_satellite_count == 0 && line_count == 1 {
            4
        } else {
            last_line_satellite_count
        };
        clerk::trace!("Gsv::new: last_line_satellite_count={last_line_satellite_count}");

        ctx.rest_str();
        let mut satellites = Vec::with_capacity(satellite_count);
        //first n-1 lines
        for _ in 0..line_count - 1 {
            for _ in 0..3 {
                let id = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
                let elevation_degrees =
                    ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
                let azimuth_degree = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
                let snr = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
                satellites.push(Satellite {
                    id,
                    elevation_degrees,
                    azimuth_degree,
                    snr,
                });
            }
            // fourth satellite in the line
            let id = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
            let elevation_degrees = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
            let azimuth_degree = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
            let snr = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_STAR).parse_opt();
            satellites.push(Satellite {
                id,
                elevation_degrees,
                azimuth_degree,
                snr,
            });
            ctx.skip(&UNTIL_COMMA)
                .skip(&CHAR_COMMA)
                .skip(&UNTIL_COMMA)
                .skip(&CHAR_COMMA)
                .skip(&UNTIL_COMMA)
                .skip(&CHAR_COMMA)
                .skip(&UNTIL_COMMA);
        }
        //last line
        if last_line_satellite_count != 0 {
            for _ in 0..(last_line_satellite_count - 1) {
                let id = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
                let elevation_degrees =
                    ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
                let azimuth_degree = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
                let snr = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
                satellites.push(Satellite {
                    id,
                    elevation_degrees,
                    azimuth_degree,
                    snr,
                });
            }
            // fourth satellite in the line
            let id = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
            let elevation_degrees = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
            let azimuth_degree = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
            let snr = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_STAR).parse_opt();
            satellites.push(Satellite {
                id,
                elevation_degrees,
                azimuth_degree,
                snr,
            });
        }

        Ok(Self { talker, satellites })
    }
}

impl fmt::Debug for Gsv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("GSV");
        ds.field("talker", &self.talker);
        ds.field("count", &self.satellites.len());
        ds.field("satellites", &self.satellites);

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use clerk::init_log_with_level;
    use clerk::tracing::level_filters::LevelFilter;

    use super::*;
    #[test]
    fn test_new_gsv() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPGSV,3,1,10,25,68,053,47,21,59,306,49,29,56,161,49,31,36,265,49*79\r\n$GPGSV,3,2,10,12,29,048,49,05,22,123,49,18,13,000,49,01,00,000,49*72\r\n$GPGSV,3,3,10,14,00,000,03,16,00,000,27*7C";
        let mut ctx = StrParserContext::new();
        let gsv = Gsv::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{:?}", gsv);
        assert_eq!(gsv.talker, Talker::GP);
        assert_eq!(gsv.satellites.len(), 10);
        //line1
        assert_eq!(gsv.satellites[0].id, Some(25));
        assert_eq!(gsv.satellites[0].elevation_degrees, Some(68));
        assert_eq!(gsv.satellites[0].azimuth_degree, Some(53));
        assert_eq!(gsv.satellites[0].snr, Some(47));
        assert_eq!(gsv.satellites[1].id, Some(21));
        assert_eq!(gsv.satellites[1].elevation_degrees, Some(59));
        assert_eq!(gsv.satellites[1].azimuth_degree, Some(306));
        assert_eq!(gsv.satellites[1].snr, Some(49));
        assert_eq!(gsv.satellites[2].id, Some(29));
        assert_eq!(gsv.satellites[2].elevation_degrees, Some(56));
        assert_eq!(gsv.satellites[2].azimuth_degree, Some(161));
        assert_eq!(gsv.satellites[2].snr, Some(49));
        assert_eq!(gsv.satellites[3].id, Some(31));
        assert_eq!(gsv.satellites[3].elevation_degrees, Some(36));
        assert_eq!(gsv.satellites[3].azimuth_degree, Some(265));
        assert_eq!(gsv.satellites[3].snr, Some(49));
        //line2
        assert_eq!(gsv.satellites[4].id, Some(12));
        assert_eq!(gsv.satellites[4].elevation_degrees, Some(29));
        assert_eq!(gsv.satellites[4].azimuth_degree, Some(48));
        assert_eq!(gsv.satellites[4].snr, Some(49));
        assert_eq!(gsv.satellites[5].id, Some(5));
        assert_eq!(gsv.satellites[5].elevation_degrees, Some(22));
        assert_eq!(gsv.satellites[5].azimuth_degree, Some(123));
        assert_eq!(gsv.satellites[5].snr, Some(49));
        assert_eq!(gsv.satellites[6].id, Some(18));
        assert_eq!(gsv.satellites[6].elevation_degrees, Some(13));
        assert_eq!(gsv.satellites[6].azimuth_degree, Some(0));
        assert_eq!(gsv.satellites[6].snr, Some(49));
        assert_eq!(gsv.satellites[7].id, Some(1));
        assert_eq!(gsv.satellites[7].elevation_degrees, Some(0));
        assert_eq!(gsv.satellites[7].azimuth_degree, Some(0));
        assert_eq!(gsv.satellites[7].snr, Some(49));
        //line3
        assert_eq!(gsv.satellites[8].id, Some(14));
        assert_eq!(gsv.satellites[8].elevation_degrees, Some(0));
        assert_eq!(gsv.satellites[8].azimuth_degree, Some(0));
        assert_eq!(gsv.satellites[8].snr, Some(3));
        assert_eq!(gsv.satellites[9].id, Some(16));
        assert_eq!(gsv.satellites[9].elevation_degrees, Some(0));
        assert_eq!(gsv.satellites[9].azimuth_degree, Some(0));
        assert_eq!(gsv.satellites[9].snr, Some(27));

        Ok(())
    }
    #[test]
    fn test_new_gsv_one_line() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPGSV,1,1,4,02,35,291,,03,09,129,,05,14,305,,06,38,226,*4E";
        let mut ctx = StrParserContext::new();
        let gsv = Gsv::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{:?}", gsv);
        assert_eq!(gsv.talker, Talker::GP);
        assert_eq!(gsv.satellites.len(), 4);
        assert_eq!(gsv.satellites[0].id, Some(2));
        assert_eq!(gsv.satellites[0].elevation_degrees, Some(35));
        assert_eq!(gsv.satellites[0].azimuth_degree, Some(291));
        assert!(gsv.satellites[0].snr.is_none());
        assert_eq!(gsv.satellites[1].id, Some(3));
        assert_eq!(gsv.satellites[1].elevation_degrees, Some(9));
        assert_eq!(gsv.satellites[1].azimuth_degree, Some(129));
        assert!(gsv.satellites[1].snr.is_none());
        assert_eq!(gsv.satellites[2].id, Some(5));
        assert_eq!(gsv.satellites[2].elevation_degrees, Some(14));
        assert_eq!(gsv.satellites[2].azimuth_degree, Some(305));
        assert!(gsv.satellites[2].snr.is_none());
        assert_eq!(gsv.satellites[3].id, Some(6));
        assert_eq!(gsv.satellites[3].elevation_degrees, Some(38));
        assert_eq!(gsv.satellites[3].azimuth_degree, Some(226));
        assert!(gsv.satellites[3].snr.is_none());

        Ok(())
    }
}
