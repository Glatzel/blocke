pub mod filters;
pub mod rules;

// pub trait IStrParser {
//     /// Parse a line or frame of data into a structured message.
//     fn parse(&self, input: &str) -> Self;
// }
pub struct IStrParserContext<'a> {
    full: &'a str,
    rest: &'a str,
}
impl<'a> IStrParserContext<'a> {
    pub fn new(sentence: &'a str) -> Self {
        Self {
            full: sentence,
            rest: sentence,
        }
    }
    pub fn full_str(&self) -> &'a str { self.full }
    pub fn rest_str(&self) -> &'a str { self.rest }
}

impl<'a> IStrParserContext<'a> {
    pub fn take<R, O>(&mut self, rule: R) -> Option<O>
    where
        R: rules::IRule<'a, O>,
    {
        match rule.apply_rule(self.rest) {
            Some(result) => {
                self.rest = result.1;
                Some(result.0)
            }
            None => None,
        }
    }
    pub fn take_strict<R, O>(&mut self, rule: R) -> miette::Result<O>
    where
        R: rules::IRule<'a, O>,
    {
        match self.take(rule) {
            Some(s) => Ok(s),
            None => miette::bail!("input string is shorter than requested count"),
        }
    }
}

impl<'a> IStrParserContext<'a> {
    pub fn skip<R, O>(&mut self, rule: R) -> &mut Self
    where
        R: rules::IRule<'a, O>,
    {
        self.take(rule);
        self
    }
    pub fn skip_strict<R, O>(&mut self, rule: R) -> miette::Result<&mut Self>
    where
        R: rules::IRule<'a, O>,
    {
        self.take_strict(rule)?;
        Ok(self)
    }
}
