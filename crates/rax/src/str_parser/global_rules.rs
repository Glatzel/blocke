use crate::str_parser::StrParserContext;

pub trait IStrGlobalRules<'a, O> {
    fn name(&self) -> &str;
    fn apply(&self, ctx: &StrParserContext, input: &'a str) -> O;
}
