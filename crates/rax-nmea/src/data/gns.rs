use std::fmt::Debug;

use rax_parser::str_parser::StrParserContext;

use crate::data::Talker;
use crate::macros::readonly_struct;
readonly_struct!(
    Gns,
    "Gns",
    {talker: Talker}
);

impl Gns {
    pub fn new(_ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        Ok(Gns { talker })
    }
}
impl Debug for Gns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gns").field("talker", &self.talker).finish()
    }
}
