mod byte_count;
pub use byte_count::*;
mod char_count;
pub use char_count::*;
mod until;
pub use until::*;
mod charset;
pub use charset::*;
mod char;
pub use self::char::*;
pub trait IStrTakeRule<'a, O> {
    fn name(&self) -> &str;
    fn apply(&self, input: &'a str) -> Option<(O, &'a str)>;
}
pub trait IStrGlobalRule<'a, O> {
    fn name(&self) -> &str;
    fn apply(&self, input: &'a str) -> O;
}
