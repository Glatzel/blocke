pub mod filters;
pub mod rules;

mod parse_opt;
pub use parse_opt::*;
pub use rules::{IRule, IStrFlowRule, IStrGlobalRule};

pub struct StrParserContext<'a> {
    full: &'a str,
    rest: &'a str,
}

impl<'a> StrParserContext<'a> {
    pub fn new() -> Self { Self { full: "", rest: "" } }
    pub fn init(&mut self, input: &'a str) -> &mut Self {
        self.full = input;
        self.rest = input;
        self
    }
    pub fn clean(&mut self) -> &mut Self {
        self.full = "";
        self.rest = "";
        self
    }
    pub fn full_str(&self) -> &str { self.full }
    pub fn rest_str(&self) -> &str { self.rest }
}

impl<'a> StrParserContext<'a> {
    pub fn take<R, O>(&mut self, rule: &R) -> Option<O>
    where
        R: rules::IStrFlowRule<'a, O>,
    {
        match rule.apply(self.rest) {
            Some(result) => {
                self.rest = result.1;
                Some(result.0)
            }
            None => None,
        }
    }
    pub fn take_strict<R, O>(&mut self, rule: &R) -> miette::Result<O>
    where
        R: rules::IStrFlowRule<'a, O>,
    {
        match self.take(rule) {
            Some(s) => Ok(s),
            None => miette::bail!("take fail"),
        }
    }
}

impl<'a> StrParserContext<'a> {
    pub fn skip<R, O>(&mut self, rule: &R) -> &mut Self
    where
        R: rules::IStrFlowRule<'a, String>,
    {
        self.take(rule);
        self
    }
    pub fn skip_strict<R, O>(&mut self, rule: &R) -> miette::Result<&mut Self>
    where
        R: rules::IStrFlowRule<'a, O>,
    {
        self.take_strict(rule)?;
        Ok(self)
    }
}
impl<'a> StrParserContext<'a> {
    pub fn global<R, O>(&'a mut self, rule: R) -> O
    where
        R: IStrGlobalRule<'a, O>,
    {
        rule.apply(&self.full)
    }
}
