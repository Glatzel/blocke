mod byte_count;
pub use byte_count::*;
mod char_count;
pub use char_count::*;
mod until;
pub use until::*;
mod one_of_char_set;
pub use one_of_char_set::*;
mod char;
pub use self::char::*;

pub trait IRule {
    fn name(&self) -> &str;
}
pub trait IStrFlowRule<'a, O>: IRule {
    fn apply(&self, input: &'a str) -> Option<(O, &'a str)>;
}
pub trait IStrGlobalRule<'a, O>: IRule {
    fn apply(&self, input: &'a str) -> O;
}
