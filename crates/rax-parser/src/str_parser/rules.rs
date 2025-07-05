mod byte_count;
pub use byte_count::*;
mod char_count;
pub use char_count::*;
mod until;
pub use until::*;
mod one_in_char_set;
pub use one_in_char_set::*;
mod char;
pub use self::char::*;
mod until_one_in_char_set;
pub use until_one_in_char_set::*;

pub trait IRule {
    fn name(&self) -> &str;
}
pub trait IStrFlowRule<'a>: IRule {
    type Output;
    fn apply(&self, input: &'a str) -> (Option<Self::Output>, &'a str);
}
pub trait IStrGlobalRule<'a>: IRule {
    type Output;
    fn apply(&self, input: &'a str) -> Self::Output;
}
