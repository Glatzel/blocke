use std::fmt;

use rax::str_parser::{IStrGlobalRule, ParseOptExt, StrParserContext};
use serde::{Deserialize, Serialize};

use crate::data::Talker;
use crate::macros::readonly_struct;
use crate::rules::*;

/// Represents a single satellite's data in a GSV sentence.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Satellite {
    /// Satellite ID, typically a number from 1 to 32.
    id: Option<u16>,
    /// Elevation in degrees.
    elevation_degrees: Option<u8>,
    /// Azimuth in degrees.
    azimuth_degree: Option<u16>,
    /// Signal-to-noise ratio.
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
    Gsv,
    "Gsv",
    {talker: Talker},
    {
        satellites: Vec<Satellite>,
        "Satellite data"
    }
);

impl Gsv {
    /// Parse a GSV sentence (possibly multi-line) into a Gsv struct.
    pub fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        clerk::trace!("Gsv::new: sentence='{}'", ctx.full_str());
        // Validate each line with NMEA_VALIDATE
        for l in ctx.full_str().lines() {
            NMEA_VALIDATE.apply(l)?;
        }

        // Count the number of lines and satellites
        let line_count = ctx.full_str().lines().count();
        clerk::trace!("Gsv::new: line_count={line_count}");

        // The first line contains the talker, number of lines, and number of satellites
        let satellite_count = ctx
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .take(&UNTIL_COMMA)
            .parse_opt::<usize>()
            .expect("Cannot get the count of satellites.");
        clerk::trace!("Gsv::new: satellite_count={satellite_count}");

        // The last line may have fewer than 4 satellites, so we calculate how many
        // satellites are in the last line based on the total count.
        let last_line_satellite_count = {
            let rem = satellite_count % 4;
            if rem == 0 && line_count == 1 && satellite_count != 0 {
                4
            } else {
                rem
            }
        };
        clerk::trace!("Gsv::new: last_line_satellite_count={last_line_satellite_count}");

        let mut satellites = Vec::with_capacity(satellite_count);
        // Parse all but the last line (each has 4 satellites)
        for _ in 0..line_count - 1 {
            for _ in 0..3 {
                satellites.push(Self::parse_satellite(ctx, false)?);
            }
            satellites.push(Self::parse_satellite(ctx, true)?);
            // Skip any extra fields after the 4th satellite in the line
            ctx.skip(&UNTIL_COMMA)
                .skip(&CHAR_COMMA)
                .skip(&UNTIL_COMMA)
                .skip(&CHAR_COMMA)
                .skip(&UNTIL_COMMA)
                .skip(&CHAR_COMMA)
                .skip(&UNTIL_COMMA);
        }

        // Parse the last line (may have fewer than 4 satellites)
        if last_line_satellite_count != 0 {
            for _ in 0..(last_line_satellite_count - 1) {
                satellites.push(Self::parse_satellite(ctx, false)?);
            }
            satellites.push(Self::parse_satellite(ctx, true)?);
        }

        Ok(Self { talker, satellites })
    }

    /// Helper to parse a single satellite entry.
    /// If `last` is true, the SNR field is terminated by a star.
    fn parse_satellite(ctx: &mut StrParserContext, last: bool) -> miette::Result<Satellite> {
        let id = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let elevation_degrees = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let azimuth_degree = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let snr = if last {
            ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_STAR).parse_opt()
        } else {
            ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt()
        };
        Ok(Satellite {
            id,
            elevation_degrees,
            azimuth_degree,
            snr,
        })
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
        // line 1
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
        // line 2
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
        // line 3
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
    fn test_new_gsv_4() -> miette::Result<()> {
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

    #[test]
    fn test_new_gsv_3() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPGSV,1,1,3,02,35,291,,03,09,129,,05,14,305,*72";
        let mut ctx = StrParserContext::new();
        let gsv = Gsv::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{:?}", gsv);
        assert_eq!(gsv.talker, Talker::GP);
        assert_eq!(gsv.satellites.len(), 3);
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
        Ok(())
    }
    #[test]
    fn test_new_gsv_0() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPGSV,1,1,0,*65";
        let mut ctx = StrParserContext::new();
        let gsv = Gsv::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{:?}", gsv);
        assert_eq!(gsv.talker, Talker::GP);
        assert_eq!(gsv.satellites.len(), 0);
        Ok(())
    }
}
