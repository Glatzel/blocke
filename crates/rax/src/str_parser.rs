pub mod filters;
pub mod rules;
use std::sync::{LazyLock, Mutex, MutexGuard};
mod parse_opt;
pub use parse_opt::*;
pub use rules::{IRule, IStrFlowRule, IStrGlobalRule};

pub static STR_PARSER_CONTEXT: LazyLock<Mutex<StrParserContext>> = LazyLock::new(|| {
    Mutex::new(StrParserContext {
        full: String::new(),
        rest: "",
    })
});
pub struct StrParserContext<'a> {
    full: String,
    rest: &'a str,
}

impl<'a> StrParserContext<'a> {
    pub fn new(sentence: String) -> MutexGuard<'static, StrParserContext<'a>> {
        let mut ctx = STR_PARSER_CONTEXT.lock().unwrap();
        ctx.full = sentence;
        ctx.rest = ctx.full.as_str();
        ctx
    }
    pub fn reset(&'a mut self) -> &Self {
        self.rest = &self.full;
        self
    }

    pub fn full_str(&self) -> &str { self.full.as_str() }
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
