use crate::NavigationSystem;

pub trait INmeaData: Sized + Clone {
    fn parse_sentense(sentense: &str, navigation_system: NavigationSystem) -> miette::Result<Self>;
}
