use crate::primitives::NavigationSystem;

pub trait INmeaData: Sized {
    fn parse_sentence(sentence: &str, navigation_system: NavigationSystem) -> miette::Result<Self>;
}
pub trait INmeaParser {
    fn parse_sentence(&mut self) -> miette::Result<&mut Self>;
}
