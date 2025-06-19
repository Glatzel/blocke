mod count;
pub use count::*;
pub trait IRule<'a, O> {
    fn name(&self) -> &str;
    fn apply_rule(&self, input: &'a str) -> Option<(O, &'a str)>;
}
