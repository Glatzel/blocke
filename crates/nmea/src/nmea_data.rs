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
pub use vtg::*;
pub use zda::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum NmeaDataType {
    DHV,
    GGA,
    GLL,
    GSA,
    VTG,
    ZDA,
    Other,
}
impl FromStr for NmeaDataType {
    type Err = miette::Report;

    fn from_str(sentense: &str) -> Result<Self, Self::Err> {
        if sentense.len() < 6 {
            miette::bail!("Invalid sentense: {}", sentense);
        }
        let out = match &sentense[3..6] {
            "DHV" => Self::DHV,
            "GGA" => Self::GGA,
            "GLL" => Self::GLL,
            "GSA" => Self::GSA,

            "VTG" => Self::VTG,
            "ZDA" => Self::ZDA,

            _ => Self::Other,
        };
        Ok(out)
    }
}
#[derive(Clone)]
pub enum GenericNmeaData {
    DHV(crate::nmea_data::Dhv),
    GGA(crate::nmea_data::Gga),
    GLL(crate::nmea_data::Gll),
    GSA(crate::nmea_data::Gsa),
    VTG(crate::nmea_data::Vtg),
    ZDA(crate::nmea_data::Zda),

    Other(String),
}
