use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};
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
