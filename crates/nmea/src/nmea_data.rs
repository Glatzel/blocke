mod dhv;
mod gga;
mod gll;
mod gsa;
mod vtg;
mod zda;

use std::str::FromStr;

pub use dhv::*;
pub use gga::*;
pub use gll::*;
pub use gsa::*;
use miette::IntoDiagnostic;
pub use vtg::*;
pub use zda::*;

use crate::{INmeaData, NavigationSystem};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum GenericNmeaData {
    DHV(crate::nmea_data::Dhv),
    GGA(crate::nmea_data::Gga),
    GLL(crate::nmea_data::Gll),
    GSA(crate::nmea_data::Gsa),
    // GSV(crate::nmea_data::Gsv),
    VTG(crate::nmea_data::Vtg),
    ZDA(crate::nmea_data::Zda),

    Other(String),
}
impl TryFrom<&str> for GenericNmeaData {
    type Error = miette::Report;

    fn try_from(sentense: &str) -> miette::Result<Self> {
        if sentense.len() < 6 {
            miette::bail!("Invalid sentense: {}", sentense);
        }
        let navigation_system = NavigationSystem::from_str(&sentense[1..3]).into_diagnostic()?;
        let out = match &sentense[3..6] {
            "DHV" => Self::DHV(Dhv::parse_sentense(sentense, navigation_system)?),
            "GGA" => Self::GGA(Gga::parse_sentense(sentense, navigation_system)?),
            "GLL" => Self::GLL(Gll::parse_sentense(sentense, navigation_system)?),
            "GSA" => Self::GSA(Gsa::parse_sentense(sentense, navigation_system)?),
            // "GSV" => Ok(Self::GSV),
            "VTG" => Self::VTG(Vtg::parse_sentense(sentense, navigation_system)?),
            "ZDA" => Self::ZDA(Zda::parse_sentense(sentense, navigation_system)?),

            other => GenericNmeaData::Other(other.to_string()),
        };
        Ok(out)
    }
}
