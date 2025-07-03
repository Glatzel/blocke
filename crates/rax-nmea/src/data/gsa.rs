use std::fmt;
use std::str::FromStr;

use rax_parser::str_parser::{ParseOptExt, StrParserContext};
use serde::{Deserialize, Serialize};

use crate::data::{INmeaData, Satellite, SystemId, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum GsaSelectionMode {
    Manual,
    Automatic,
}
impl FromStr for GsaSelectionMode {
    type Err = miette::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Automatic),
            "M" => Ok(Self::Manual),
            other => miette::bail!("Unknown GsaSelectionMode: {}", other),
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum GsaMode {
    NoFix,
    Fix2D,
    Fix3D,
}
impl FromStr for GsaMode {
    type Err = miette::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::NoFix),
            "2" => Ok(Self::Fix2D),
            "3" => Ok(Self::Fix3D),
            other => miette::bail!("Unknown GsaMode: {}", other),
        }
    }
}
readonly_struct!(
    Gsa ,
    "Gsa",
    {talker: Talker},

    {selection_mode: Option<GsaSelectionMode>},
    {mode : Option<GsaMode>},
    {satellite_ids:Vec<u8>},
    {pdop: Option<f64>},
    {hdop: Option<f64>},
    {vdop: Option<f64>},
    {system_id:Option<SystemId>}
);
impl INmeaData for Gsa {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;

        let selection_mode = ctx
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .take(&UNTIL_COMMA)
            .parse_opt();
        let mode = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let mut satellite_ids = Vec::with_capacity(12);
        for _ in 0..12 {
            match ctx
                .skip_strict(&CHAR_COMMA)?
                .take(&UNTIL_COMMA)
                .parse_opt::<u8>()
            {
                Some(sat_id) => satellite_ids.push(sat_id),
                None => continue,
            }
        }

        let pdop = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let hdop = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let vdop = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let system_id = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_STAR).parse_opt();

        Ok(Gsa {
            talker,
            selection_mode,
            mode,
            satellite_ids,
            pdop,
            hdop,
            vdop,
            system_id,
        })
    }
}

impl fmt::Debug for Gsa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("GSA");
        ds.field("talker", &self.talker);

        if let Some(selection_mode) = self.selection_mode {
            ds.field("selection_mode", &selection_mode);
        }
        if let Some(mode) = self.mode {
            ds.field("mode", &mode);
        }
        if !self.satellite_ids.is_empty() {
            ds.field("satellite_ids", &self.satellite_ids);
        }
        if let Some(pdop) = self.pdop {
            ds.field("pdop", &pdop);
        }
        if let Some(hdop) = self.hdop {
            ds.field("hdop", &hdop);
        }
        if let Some(vdop) = self.vdop {
            ds.field("vdop", &vdop);
        }
        if let Some(system_id) = self.system_id {
            ds.field("system_id", &system_id);
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
    fn test_new_gsa() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GNGSA,A,3,05,07,13,14,15,17,19,23,24,,,,1.0,0.7,0.7,1*38";
        let mut ctx = StrParserContext::new();
        let gsa = Gsa::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", gsa);
        assert_eq!(gsa.talker, Talker::GN);
        assert_eq!(gsa.selection_mode.unwrap(), GsaSelectionMode::Automatic);
        assert_eq!(gsa.mode.unwrap(), GsaMode::Fix3D);
        assert_eq!(gsa.satellite_ids, vec![5, 7, 13, 14, 15, 17, 19, 23, 24]);
        assert_approx_eq!(f64, gsa.pdop.unwrap(), 1.0);
        assert_approx_eq!(f64, gsa.hdop.unwrap(), 0.7);
        assert_approx_eq!(f64, gsa.vdop.unwrap(), 0.7);

        Ok(())
    }
}
