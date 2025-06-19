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
pub trait IStrFlowRule<'a, O> {
    fn name(&self) -> &str;
    fn apply(&self, ctx: &'a super::StrParserContext) -> Option<(O, &'a str)>;
}
