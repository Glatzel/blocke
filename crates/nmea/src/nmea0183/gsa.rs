use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::utils::readonly_struct;
use crate::{NavigationSystem, SystemId};
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
    "",
    {navigation_system: NavigationSystem},
    {is_valid: bool},

    {selection_mode: Option<GsaSelectionMode>},
    {mode : Option<GsaMode>},
    {satellite_ids:Vec<u8>},
    {pdop: Option<f64>},
    {hdop: Option<f64>},
    {vdop: Option<f64>},
    {system_id:Option<SystemId>}
);
impl crate::parser::NmeaParser {
    pub fn new_gsa(sentence: &str) -> miette::Result<Gsa> {
        let parts: Vec<&str> = Self::get_sentense_parts(sentence);
        Ok(Gsa::new(
            Self::get_navigation_system(&sentence)?,
            Self::is_valid(sentence),
            Self::parse_primitive(&parts, 1)?,
            Self::parse_primitive(&parts, 2)?,
            (3..15)
                .map(|i| Self::parse_primitive(&parts, i).unwrap())
                .filter_map(|f| f)
                .collect(),
            Self::parse_primitive(&parts, 15)?,
            Self::parse_primitive(&parts, 16)?,
            Self::parse_primitive(&parts, 17)?,
            Self::parse_primitive(&parts, 18)?,
        ))
    }
}
#[cfg(test)]
mod test {
    use test_utils::init_log;

    use crate::parser::NmeaParser;
    #[test]
    fn test_new_gsa() -> miette::Result<()> {
        init_log();
        let s = "$GNGSA,A,3,80,71,73,79,69,,,,,,,,1.83,1.09,1.47*17";
        for (i, v) in NmeaParser::get_sentense_parts(s).iter().enumerate() {
            println!("{i}:{v}");
        }
        let gsa = NmeaParser::new_gsa(s)?;
        println!("{:?}", gsa);
        assert!(gsa.is_valid);
        Ok(())
    }
}
