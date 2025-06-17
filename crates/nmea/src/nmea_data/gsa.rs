use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::nmea_data::NavigationSystem;
use crate::utils::{readonly_struct, *};
use crate::{INmeaData, SystemId};
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
impl INmeaData for Gsa {
    fn parse_sentense(sentence: &str, navigation_system: NavigationSystem) -> miette::Result<Gsa> {
        let parts: Vec<&str> = get_sentense_parts(sentence);
        Ok(Gsa {
            navigation_system,
            is_valid: is_valid(sentence),
            selection_mode: parse_primitive(&parts, 1)?,
            mode: parse_primitive(&parts, 2)?,
            satellite_ids: (3..15)
                .map(|i| parse_primitive(&parts, i).unwrap())
                .filter_map(|f| f)
                .collect(),
            pdop: parse_primitive(&parts, 15)?,
            hdop: parse_primitive(&parts, 16)?,
            vdop: parse_primitive(&parts, 17)?,
            system_id: parse_primitive(&parts, 18)?,
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
        for (i, v) in get_sentense_parts(s).iter().enumerate() {
            println!("{i}:{v}");
        }
        let gsa = Gsa::parse_sentense(s, NavigationSystem::GN)?;
        println!("{:?}", gsa);
        assert!(gsa.is_valid);
        Ok(())
    }
}
