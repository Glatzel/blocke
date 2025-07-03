use std::fmt::Debug;

use rax_parser::str_parser::StrParserContext;

use crate::data::Talker;
use crate::macros::readonly_struct;
readonly_struct!(
    Grs,
    "Grs",
    {talker: Talker}
);

impl Grs {
    pub fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        Ok(Grs { talker })
    }
}
impl Debug for Grs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Grs").field("talker", &self.talker).finish()
    }
}
