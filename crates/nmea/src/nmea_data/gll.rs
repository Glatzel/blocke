use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::nmea_data::NavigationSystem;
use crate::utils::{readonly_struct, *};
use crate::{FaaMode, INmeaData};
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GllDataValid {
    Valid,
    Invalid,
}
impl FromStr for GllDataValid {
    type Err = miette::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Valid),
            "V" => Ok(Self::Invalid),
            other => miette::bail!("Unknown GllDataValid {}", other),
        }
    }
}
readonly_struct!(
    Gll ,
    "",
    {navigation_system: NavigationSystem},
    {is_valid: bool},

    {lat: Option<f64>},
    {lon: Option<f64>},
    {utc_time: Option<chrono::DateTime<chrono::Utc>>},
    {data_valid: Option<GllDataValid>},
    {faa_mode: Option<FaaMode>}
);
impl INmeaData for Gll {
    fn parse_sentence(sentence: &str, navigation_system: NavigationSystem) -> miette::Result<Gll> {
        let parts: Vec<&str> = get_sentence_parts(sentence);
        Ok(Gll {
            navigation_system,
            is_valid: is_valid(sentence),
            lat: parse_latitude(&parts, 1, 2)?,
            lon: parse_longitude(&parts, 3, 4)?,
            utc_time: parse_utc(&parts, 5)?,
            data_valid: parse_primitive(&parts, 6)?,
            faa_mode: parse_primitive(&parts, 7)?,
        })
    }
}

#[cfg(test)]
mod test {
    use test_utils::init_log;

    use super::*;
    #[test]
    fn test_new_ggl() -> miette::Result<()> {
        init_log();
        let s = "$GPGLL,2959.9925,S,12000.0090,E,235316.000,A,A*4E";
        for (i, v) in get_sentence_parts(s).iter().enumerate() {
            println!("{i}:{v}");
        }
        let gll = Gll::parse_sentence(s, NavigationSystem::GN)?;
        println!("{:?}", gll);
        assert!(gll.is_valid);
        Ok(())
    }
}
