use crate::nmea_data::NavigationSystem;

pub trait INmeaData: Sized + Clone {
    fn parse_sentence(sentence: &str, navigation_system: NavigationSystem) -> miette::Result<Self>;
}
