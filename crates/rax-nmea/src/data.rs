mod dhv;
mod gga;
mod gll;
mod gsa;
mod gst;
mod rmc;
mod vtg;
mod zda;

use std::fmt::Display;
use std::str::FromStr;

pub use dhv::*;
pub use gga::*;
pub use gll::*;
pub use gsa::*;
pub use gst::*;
use rax_parser::str_parser::StrParserContext;
pub use rmc::*;
use serde::{Deserialize, Serialize};
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
    GGA,
    GLL,
    GSA,
    GST,
    GSV,
    RMC,
    VTG,
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
            "GGA" => Self::GGA,
            "GLL" => Self::GLL,
            "GSA" => Self::GSA,
            "GST" => Self::GST,
            "GSV" => Self::GSV,
            "RMC" => Self::RMC,
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
            Self::DHV => "DHV,",
            Self::GGA => "GGA,",
            Self::GLL => "GLL,",
            Self::GSA => "GSA,",
            Self::GST => "GST,",
            Self::GSV => "GSV,",
            Self::RMC => "RMC,",
            Self::VTG => "VTG,",
            Self::ZDA => "ZDA,",
        };
        write!(f, "{}", s)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, Hash, Eq)]
pub enum Talker {
    ///BeiDou (China)
    BD,
    ///GLONASS, according to IEIC 61162-1
    GL,
    ///Combination of multiple satellite systems (NMEA 1083)
    GN,
    ///Global Positioning System receiver
    GP,
}

impl FromStr for Talker {
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
impl Display for Talker {
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
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum FaaMode {
    Autonomous,
    Differential,
    Estimated,
    ManualInput,
    NotValid,
    Simulator,
}
impl FromStr for FaaMode {
    type Err = miette::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Autonomous),
            "D" => Ok(Self::Differential),
            "E" => Ok(Self::Estimated),
            "M" => Ok(Self::ManualInput),
            "S" => Ok(Self::Simulator),
            "N" => Ok(Self::NotValid),
            other => miette::bail!("Unknown GgaQualityIndicator {}", other),
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
            other => miette::bail!("Unknown GgaQualityIndicator {}", other),
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
            other => miette::bail!("Unknown GllDataValid {}", other),
        }
    }
}
