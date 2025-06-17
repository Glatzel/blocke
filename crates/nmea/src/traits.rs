use crate::NavigationSystem;

pub trait INmeaData {
    fn parse_sentense(sentense: &str, navigation_system: NavigationSystem) -> miette::Result<Self>
    where
        Self: Sized;
}
