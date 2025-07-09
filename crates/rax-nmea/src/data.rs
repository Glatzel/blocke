mod dhv;
mod gbs;
mod gga;
mod gll;
mod gns;
mod grs;
mod gsa;
mod gst;
mod gsv;
mod rmc;
mod txt;
mod vtg;
mod zda;
use std::fmt::Display;
use std::str::FromStr;

pub use dhv::*;
pub use gbs::*;
pub use gga::*;
pub use gll::*;
pub use gns::*;
pub use grs::*;
pub use gsa::*;
pub use gst::*;
pub use gsv::*;
use rax::str_parser::StrParserContext;
pub use rmc::*;
use serde::{Deserialize, Serialize};
pub use txt::*;
pub use vtg::*;
pub use zda::*;
pub trait INmeaData {
    fn new(ctx: &mut StrParserContext, navigation_system: Talker) -> miette::Result<Self>
    where
        Self: Sized;
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Identifier {
    DHV,
    ///GPS Satellite Fault Detection
    GBS,
    ///Global Positioning System Fix Data
    GGA,
    ///Geographic Position - Latitude/Longitude
    GLL,
    ///Fix data
    GNS,
    ///GPS Range Residuals
    GRS,
    ///GPS Pseudorange Noise Statistics
    GSA,
    ///GPS DOP and active satellites
    GST,
    ///Satellites in viewR
    GSV,
    ///Recommended Minimum Navigation Information
    RMC,
    Txt,
    ///Track made good and Ground speed
    VTG,
    ///Time & Date - UTC, day, month, year and local time zone
    ZDA,
}
impl FromStr for Identifier {
    type Err = miette::Report;

    fn from_str(sentence: &str) -> Result<Self, Self::Err> {
        if sentence.len() < 6 {
            miette::bail!("Invalid sentence: {}", sentence);
        }
        let out = match &sentence[3..6] {
            "DHV" => Self::DHV,
            "GBS" => Self::GBS,
            "GGA" => Self::GGA,
            "GLL" => Self::GLL,
            "GNS" => Self::GNS,
            "GRS" => Self::GRS,
            "GSA" => Self::GSA,
            "GST" => Self::GST,
            "GSV" => Self::GSV,
            "RMC" => Self::RMC,
            "TXT" => Self::Txt,
            "VTG" => Self::VTG,
            "ZDA" => Self::ZDA,

            s => miette::bail!("Unknown identifier: {}", s),
        };
        Ok(out)
    }
}
impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::DHV => "DHV",
            Self::GBS => "GBS",
            Self::GGA => "GGA",
            Self::GLL => "GLL",
            Self::GNS => "GNS",
            Self::GRS => "GRS",
            Self::GSA => "GSA",
            Self::GST => "GST",
            Self::GSV => "GSV",
            Self::RMC => "RMC",
            Self::Txt => "TXT",
            Self::VTG => "VTG",
            Self::ZDA => "ZDA",
        };
        write!(f, "{s}")
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, Hash, Eq)]
pub enum Talker {
    ///BeiDou (China)
    BD,
    //Galileo Positioning System
    GA,
    ///GLONASS, according to IEIC 61162-1
    GL,
    ///Combination of multiple satellite systems (NMEA 1083)
    GN,
    ///Global Positioning System receiver
    GP,
    //QZSS (Quectel Quirk)
    PQ,
}

impl FromStr for Talker {
    type Err = miette::Report;

    fn from_str(sentence: &str) -> miette::Result<Self> {
        let out = match &sentence[1..3] {
            "BD" => Self::BD,
            "GA" => Self::GA,
            "GL" => Self::GL,
            "GN" => Self::GN,
            "GP" => Self::GP,
            "PQ" => Self::PQ,
            _ => miette::bail!("Unknown talker: {}", &sentence[1..3]),
        };
        Ok(out)
    }
}
impl Display for Talker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::BD => "BD",
            Self::GA => "GA",
            Self::GL => "GL",
            Self::GN => "GN",
            Self::GP => "GP",
            Self::PQ => "PQ",
        };
        write!(f, "{s}")
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum PosMode {
    Autonomous,
    Differential,
    Estimated,
    RtkFloat,
    ManualInput,
    NotValid,
    Precise,
    RtkInteger,
    Simulator,
}
impl FromStr for PosMode {
    type Err = miette::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Autonomous),
            "D" => Ok(Self::Differential),
            "E" => Ok(Self::Estimated),
            "F" => Ok(Self::RtkFloat),
            "M" => Ok(Self::ManualInput),
            "N" => Ok(Self::NotValid),
            "P" => Ok(Self::Precise),
            "R" => Ok(Self::RtkInteger),
            "S" => Ok(Self::Simulator),

            other => miette::bail!("Unknown FaaMode: {}", other),
        }
    }
}
impl TryFrom<&char> for PosMode {
    type Error = miette::Report;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Autonomous),
            'D' => Ok(Self::Differential),
            'E' => Ok(Self::Estimated),
            'F' => Ok(Self::RtkFloat),
            'M' => Ok(Self::ManualInput),
            'N' => Ok(Self::NotValid),
            'P' => Ok(Self::Precise),
            'R' => Ok(Self::RtkInteger),
            'S' => Ok(Self::Simulator),

            other => miette::bail!("Unknown FaaMode: {}", other),
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum SystemId {
    GPS = 1,
    GLONASS = 2,
    BDS = 3,
    QZSS = 4,
    NavIC = 5,
}
impl FromStr for SystemId {
    type Err = miette::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::GPS),
            "2" => Ok(Self::GLONASS),
            "3" => Ok(Self::BDS),
            "4" => Ok(Self::QZSS),
            "5" => Ok(Self::NavIC),
            other => miette::bail!("Unknown sysyemid {}", other),
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Valid,
    Invalid,
}
impl FromStr for Status {
    type Err = miette::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Valid),
            "V" => Ok(Self::Invalid),
            other => miette::bail!("Unknown status {}", other),
        }
    }
}
