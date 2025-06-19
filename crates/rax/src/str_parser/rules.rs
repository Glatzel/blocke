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
pub trait IRule {
    fn name(&self) -> &str;
}
pub trait IStrTakeRule<'a, O>: IRule {
    fn apply_take_rule(&self, input: &'a str) -> Option<(O, &'a str)>;
}
pub trait IStrSkipRule<'a, O>: IRule {
    fn apply_skip_rule(&self, input: &'a str) -> Option<(O, &'a str)>;
}
pub trait IStrGlobalRule<'a, O>: IRule {
    fn apply_global_rule(&self, input: &'a str) -> O;
}
