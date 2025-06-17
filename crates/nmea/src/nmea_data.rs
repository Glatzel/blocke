mod dhv;
mod gga;
mod gll;
mod gsa;
mod vtg;
mod zda;

use std::fmt::Display;
use std::str::FromStr;

pub use dhv::*;
pub use gga::*;
pub use gll::*;
pub use gsa::*;
use serde::{Deserialize, Serialize};
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
    Other(String),
}
impl FromStr for NmeaDataType {
    type Err = miette::Report;

    fn from_str(sentence: &str) -> Result<Self, Self::Err> {
        if sentence.len() < 6 {
            miette::bail!("Invalid sentence: {}", sentence);
        }
        let out = match &sentence[3..6] {
            "DHV" => Self::DHV,
            "GGA" => Self::GGA,
            "GLL" => Self::GLL,
            "GSA" => Self::GSA,

            "VTG" => Self::VTG,
            "ZDA" => Self::ZDA,

            _ => Self::Other(
                sentence
                    .split(",")
                    .collect::<Vec<&str>>()
                    .first()
                    .unwrap()
                    .to_string(),
            ),
        };
        Ok(out)
    }
}

impl Display for NmeaDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            NmeaDataType::DHV => "DHV",
            NmeaDataType::GGA => "GGA",
            NmeaDataType::GLL => "GLL",
            NmeaDataType::GSA => "GSA",
            NmeaDataType::VTG => "VTG",
            NmeaDataType::ZDA => "ZDA",
            NmeaDataType::Other(s) => s,
        };
        write!(f, "{}", s)
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, Hash, Eq)]
pub enum NavigationSystem {
    ///BeiDou (China)
    BD,
    ///GLONASS, according to IEIC 61162-1
    GL,
    ///Combination of multiple satellite systems (NMEA 1083)
    GN,
    ///Global Positioning System receiver
    GP,
}

impl FromStr for NavigationSystem {
    type Err = miette::Report;

    fn from_str(sentence: &str) -> miette::Result<Self> {
        let out = match &sentence[1..3] {
            "BD" => Self::BD,
            "GL" => Self::GL,
            "GN" => Self::GN,
            "GP" => Self::GP,
            _ => miette::bail!("Unknown NavigationSystem: {}", &sentence[1..3]),
        };
        Ok(out)
    }
}
impl Display for NavigationSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::BD => "BD",
            Self::GL => "GL",
            Self::GN => "GN",
            Self::GP => "GP",
        };
        write!(f, "{}", s)
    }
}
