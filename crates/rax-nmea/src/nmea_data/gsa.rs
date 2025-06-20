use std::str::FromStr;

use rax::str_parser::rules::{Char, Until};
use rax::str_parser::{ParseOptExt, StrParserContext};
use serde::{Deserialize, Serialize};

use crate::macros::readonly_struct;
use crate::nmea_data::{INmeaData, SystemId, Talker};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
    {navigation_system: Talker},

    {selection_mode: Option<GsaSelectionMode>},
    {mode : Option<GsaMode>},
    {satellite_ids:Vec<u8>},
    {pdop: Option<f64>},
    {hdop: Option<f64>},
    {vdop: Option<f64>},
    {system_id:Option<SystemId>}
);
impl INmeaData for Gsa {
    fn new(ctx: &mut StrParserContext, navigation_system: Talker) -> miette::Result<Self> {
        let char_comma = Char(&',');
        let until_comma = Until(",");
        let until_star = Until("*");

        let selection_mode = ctx
            .skip_strict(&until_comma)?
            .take(&until_comma)
            .parse_opt();
        let mode = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let satellite_ids = ctx
            .skip_strict(&char_comma)?
            .take(&until_comma)
            .map(|sats| {
                sats.split(',')
                    .filter_map(|id| id.trim().parse::<u8>().ok())
                    .collect::<Vec<u8>>()
            })
            .unwrap_or_default();
        let pdop = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let hdop = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let vdop = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let system_id = ctx.skip_strict(&char_comma)?.take(&until_star).parse_opt();

        Ok(Gsa {
            navigation_system,
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

#[cfg(test)]
mod test {
    use test_utils::init_log;

    use super::*;

    #[test]
    fn test_new_gsa() -> miette::Result<()> {
        init_log();
        let s = "$GNGSA,A,3,80,71,73,79,69,,,,,,,,1.83,1.09,1.47*17";
        let mut ctx = StrParserContext::new();
        let gsa = Gsa::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", gsa);
        Ok(())
    }
}
