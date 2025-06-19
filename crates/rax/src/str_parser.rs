pub mod filters;
pub mod flow_rules;
mod global_rules;
use std::sync::{LazyLock, Mutex, MutexGuard};

pub use flow_rules::IStrFlowRule;
pub use global_rules::IStrGlobalRules;
static STR_PARSER_CONTEXT: LazyLock<Mutex<StrParserContext>> =
    LazyLock::new(|| Mutex::new(StrParserContext { full: "", rest: "" }));
pub struct StrParserContext<'a> {
    full: &'a str,
    rest: &'a str,
}

impl<'a> StrParserContext<'a> {
    pub fn new(sentence: &'a str) -> MutexGuard<'static, StrParserContext> {
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
    fn full_str(&self) -> &'a str { self.full }
    fn rest_str(&self) -> &'a str { self.rest }
}

impl<'a> StrParserContext<'a> {
    pub fn take<R, O>(&mut self, rule: &R) -> Option<O>
    where
        R: flow_rules::IStrFlowRule<'a, O>,
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
        R: flow_rules::IStrFlowRule<'a, O>,
    {
        match self.take(rule) {
            Some(s) => Ok(s),
            None => miette::bail!("input string is shorter than requested count"),
        }
    }
}

impl<'a> StrParserContext<'a> {
    pub fn skip<R, O>(&mut self, rule: &R) -> &mut Self
    where
        R: flow_rules::IStrFlowRule<'a, O>,
    {
        self.take(rule);
        self
    }
    pub fn skip_strict<R, O>(&mut self, rule: &R) -> miette::Result<&mut Self>
    where
        R: flow_rules::IStrFlowRule<'a, O>,
    {
        self.take_strict(rule)?;
        Ok(self)
    }
}
impl<'a> StrParserContext<'a> {
    pub fn global<R, O>(&mut self, rule: R) -> O
    where
        R: global_rules::IStrGlobalRules<'a, O>,
    {
        rule.apply(self.full)
    }
}
