use std::fmt::{self};

use rax_parser::str_parser::{IStrGlobalRule, ParseOptExt, StrParserContext};
use serde::{Deserialize, Serialize};

use crate::data::Talker;
use crate::macros::readonly_struct;
use crate::rules::*;
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TxtType {
    Error = 0,
    Warn = 1,
    Info = 2,
    User = 7,
}
impl TryFrom<u8> for TxtType {
    type Error = miette::Report;
    fn try_from(s: u8) -> miette::Result<Self> {
        match s {
            0 => Ok(Self::Error),
            1 => Ok(Self::Warn),
            2 => Ok(Self::Info),
            7 => Ok(Self::User),
            _ => miette::bail!("Unknown txt type: {}", s),
        }
    }
}
impl std::fmt::Display for TxtType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TxtType::Error => "Error",
            TxtType::Warn => "Warn",
            TxtType::Info => "Info",
            TxtType::User => "User",
        };
        write!(f, "{}", s)
    }
}
readonly_struct!(
    Txt ,
    "TXT",
    {talker: Talker},

    {infos : Vec<( Option<TxtType>,Option<String>)>}
);

impl Txt {
    pub fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        clerk::trace!("Txt::new: sentence='{}'", ctx.full_str());

        for l in ctx.full_str().lines() {
            NMEA_VALIDATE.apply(l)?;
        }

        let mut infos = Vec::new();
        for _ in 0..ctx.full_str().lines().count() {
            let txt_type = ctx
                .skip_strict(&UNTIL_COMMA)?
                .skip_strict(&CHAR_COMMA)?
                .skip_strict(&UNTIL_COMMA)?
                .skip_strict(&CHAR_COMMA)?
                .skip_strict(&UNTIL_COMMA)?
                .skip_strict(&CHAR_COMMA)?
                .take(&UNTIL_COMMA)
                .parse_opt::<u8>()
                .map(TxtType::try_from)
                .and_then(Result::ok);
            let info = ctx
                .skip_strict(&CHAR_COMMA)?
                .take(&UNTIL_STAR)
                .map(|f| f.to_string());
            infos.push((txt_type, info));
            ctx.skip(&UNTIL_NEW_LINE).skip(&CHAR_NEW_LINE);
        }

        Ok(Self { talker, infos })
    }
}

impl fmt::Debug for Txt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("TXT");
        ds.field("talker", &self.talker);

        ds.field(
            "info",
            &self
                .infos
                .iter()
                .filter(|x| x.0.is_some() || x.1.is_some())
                .map(|x| match x {
                    (None, None) => panic!("Null txt info"),
                    (None, Some(i)) => i.to_string(),
                    (Some(t), None) => format!("{}: ", t),
                    (Some(t), Some(i)) => format!("{}: {}", t, i),
                })
                .collect::<Vec<String>>(),
        );

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use clerk::tracing::level_filters::LevelFilter;
    use test_utils::init_log_with_level;

    use super::*;
    #[test]
    fn test_new_zda() -> miette::Result<()> {
      init_log_with_level(LevelFilter::TRACE);
        let s = "$GPTXT,03,01,02,MA=CASIC*25\r\n$GPTXT,03,02,02,IC=ATGB03+ATGR201*70\r\n$GPTXT,03,03,02,SW=URANUS2,V2.2.1.0*1D";
        let mut ctx = StrParserContext::new();
        let txt = Txt::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{:?}", txt);
        Ok(())
    }
}
