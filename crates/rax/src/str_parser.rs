pub mod filters;
pub mod rules;

mod parse_opt;
pub use parse_opt::*;
pub use rules::{IRule, IStrFlowRule, IStrGlobalRule};

pub struct StrParserContext {
    full: String,
    rest: *const str,
}

impl Default for StrParserContext {
    fn default() -> Self { Self::new() }
}

impl StrParserContext {
    pub fn new() -> Self {
        Self {
            full: String::new(),
            rest: "",
        }
    }
    pub fn init(&mut self, input: String) -> &mut Self {
        self.full = input;
        self.rest = self.full.as_str();
        self
    }

    pub fn full_str(&self) -> &str { self.full.as_str() }
    pub fn rest_str(&self) -> &str { unsafe { &*self.rest } }
}

impl<'a> StrParserContext {
    pub fn take<R, O>(&mut self, rule: &R) -> Option<O>
    where
        R: rules::IStrFlowRule<'a, O>,
    {
        match rule.apply(unsafe { &*self.rest }) {
            (Some(result), rest) => {
                self.rest = rest;
                Some(result)
            }
            (None, rest) => {
                self.rest = rest;
                None
            }
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

impl<'a> StrParserContext {
    pub fn skip<R, O>(&mut self, rule: &R) -> &mut Self
    where
        R: rules::IStrFlowRule<'a, O>,
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
impl<'a> StrParserContext {
    pub fn global<R, O>(&'a mut self, rule: R) -> O
    where
        R: IStrGlobalRule<'a, O>,
    {
        rule.apply(&self.full)
    }
}
