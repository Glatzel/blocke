use std::fmt::Debug;

use rax_parser::str_parser::StrParserContext;

use crate::data::Talker;
use crate::macros::readonly_struct;

readonly_struct!(
    Gbs,
    "GPS Satellite Fault Detection",

   {talker: Talker}
);

impl Gbs {
    pub fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        Ok(Gbs { talker })
    }
}
impl Debug for Gbs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gbs").field("talker", &self.talker).finish()
    }
}
