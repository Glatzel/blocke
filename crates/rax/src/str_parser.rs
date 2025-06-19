pub mod rules;
mod verb;

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
