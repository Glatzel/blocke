mod char;
pub use self::char::*;

pub trait IFilter< I> {
    fn name(&self) -> &str;
    fn filter(&self, input: I) -> bool;
}
