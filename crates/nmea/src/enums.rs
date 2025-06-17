use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Hash, Eq)]
pub enum NmeaType {
    DHV,
    GGA,
    GLL,
    GSA,
    GSV,
    VTG,
    ZDA,

    Other(String),
}
impl TryFrom<&str> for NmeaType {
    type Error = miette::Report;

    fn try_from(s: &str) -> miette::Result<Self> {
        match s {
            "DHV" => Ok(Self::DHV),
            "GGA" => Ok(Self::GGA),
            "GLL" => Ok(Self::GLL),
            "GSA" => Ok(Self::GSA),
            "GSV" => Ok(Self::GSV),
            "VTG" => Ok(Self::VTG),
            "ZDA" => Ok(Self::ZDA),

            "" => miette::bail!("Empty string."),

            other => Ok(NmeaType::Other(other.to_string())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, EnumString, AsRefStr)]
pub enum NavigationSystem {
    ///BeiDou (China)
    #[strum(serialize = "BeiDou (China)", serialize = "BD")]
    BD,
    ///GLONASS, according to IEIC 61162-1
    #[strum(serialize = "GLONASS, according to IEIC 61162-1", serialize = "GL")]
    GL,
    ///Combination of multiple satellite systems (NMEA 1083)
    #[strum(
        serialize = "Combination of multiple satellite systems (NMEA 1083)",
        serialize = "GN"
    )]
    GN,
    ///Global Positioning System receiver
    #[strum(serialize = "Global Positioning System receiver", serialize = "GP")]
    GP,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_nmea0813() -> miette::Result<()> {
        //valid
        assert_eq!(NmeaType::try_from("$BDGGA")?, NmeaType::GGA);

        //other
        assert_eq!(
            NmeaType::try_from("$BDunknown")?,
            NmeaType::Other("unknown".to_string())
        );

        //invalid
        assert!(NmeaType::try_from("").is_err());
        assert!(NmeaType::try_from("$").is_err());

        Ok(())
    }
}
