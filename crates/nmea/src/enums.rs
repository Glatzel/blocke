use std::str::FromStr;

use serde::{Deserialize, Serialize};

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
