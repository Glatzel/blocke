use std::fmt::{self};

use rax_parser::str_parser::rules::{Char, Until};
use rax_parser::str_parser::{ParseOptExt, StrParserContext};

use crate::data::Talker;
use crate::macros::readonly_struct;

pub struct Satellite {
    id: u16,
    elevation_degrees: u8,
    azimuth_degree: u16,
    snr: u8,
}
readonly_struct!(
    Gsv ,
    "Gsv",
    {talker: Talker}

);

impl Gsv {
    pub fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        clerk::trace!("Txt::new: sentence='{}'", ctx.full_str());
        let mut infos = Vec::new();
        let char_comma = Char(&',');
        let until_comma = Until(",");
        let until_star = Until("*");
        let until_new_line = Until("\n");
        let char_new_line = Char(&'\n');
        for _ in 0..ctx.full_str().lines().count() {
            let txt_type = ctx
                .skip_strict(&until_comma)?
                .skip_strict(&char_comma)?
                .skip_strict(&until_comma)?
                .skip_strict(&char_comma)?
                .skip_strict(&until_comma)?
                .skip_strict(&char_comma)?
                .take(&until_comma)
                .parse_opt::<u8>()
                .map(TxtType::try_from)
                .and_then(Result::ok);
            let info = ctx
                .skip_strict(&char_comma)?
                .take(&until_star)
                .map(|f| f.to_string());
            infos.push((txt_type, info));
            ctx.skip(&until_new_line).skip(&char_new_line);
        }

        Ok(Self {
            talker,
            info: infos,
        })
    }
}

impl fmt::Debug for Txt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("ZDA");
        ds.field("talker", &self.talker);

        ds.field(
            "info",
            &self
                .info
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
    use test_utils::init_log;

    use super::*;
    #[test]
    fn test_new_zda() -> miette::Result<()> {
        init_log();
        let s = "$GPTXT,03,01,02,MA=CASIC*27\r\n$GPTXT,03,02,02,IC=ATGB03+ATGR201*71\r\n$GPTXT,03,03,02,SW=URANUS2,V2.2.1.0*1D";
        let mut ctx = StrParserContext::new();
        let gsv = Gsv::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{:?}", gsv);
        Ok(())
    }
}
