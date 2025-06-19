pub mod filters;
pub mod rules;
use std::sync::{LazyLock, Mutex, MutexGuard};

pub use rules::{IRule, IStrFlowRule, IStrGlobalRule};

pub static STR_PARSER_CONTEXT: LazyLock<Mutex<StrParserContext>> =
    LazyLock::new(|| Mutex::new(StrParserContext { full: "", rest: "" }));
pub struct StrParserContext<'a> {
    full: &'a str,
    rest: &'a str,
}

impl StrParserContext<'static> {
    pub fn new(sentence: &'static str) -> MutexGuard<'static, StrParserContext<'static>> {
        let mut ctx = STR_PARSER_CONTEXT.lock().unwrap();
        ctx.full = sentence;
        ctx.rest = sentence;
        ctx
    }
    pub fn reset(&mut self) -> &Self {
        self.rest = self.full;
        self
    }
    pub fn clean(&mut self) -> &Self {
        self.full = "";
        self.rest = "";
        self
    }
    pub fn full_str(&self) -> &'static str { self.full }
    pub fn rest_str(&self) -> &'static str { self.rest }
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
    pub fn global<R, O>(&mut self, rule: R) -> O
    where
        R: IStrGlobalRule<'a, O>,
    {
        rule.apply(self.full)
    }
}

pub trait ParseOptExt<T> {
    fn parse_opt<U: std::str::FromStr>(self) -> Option<U>;
}

impl<'a> ParseOptExt<&'a str> for Option<&'a str> {
    fn parse_opt<U: std::str::FromStr>(self) -> Option<U> { self.and_then(|s| s.parse::<U>().ok()) }
}
