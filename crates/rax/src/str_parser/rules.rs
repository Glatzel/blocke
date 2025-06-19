mod byte_count;
pub use byte_count::*;
mod char_count;
pub use char_count::*;
mod until;
pub use until::*;
mod char;
pub use char::*;

pub trait IRule<'a, O> {
    fn name(&self) -> &str;
    fn apply_rule(&self, input: &'a str) -> Option<(O, &'a str)>;
}
