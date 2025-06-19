use rax::str_parser::StrParserContext;
use rax::str_parser::rules::{Char, Until};

use crate::NmeaUtc;
use crate::macros::readonly_struct;
use crate::nmea_data::NavigationSystem;

readonly_struct!(
    Zda ,
    "Zda",
    {navigation_system: NavigationSystem},

    {utc_time: Option<chrono::DateTime<chrono::Utc>>},
    {day : Option<u8>},
    {month: Option<u8>},
    {year: Option<u16>},
    {local_zone_description: Option<i8>},
    {local_zone_minutes_description: Option<u8>}
);

impl Zda {
    pub fn new(
        sentence: &'static str,
        navigation_system: NavigationSystem,
    ) -> miette::Result<Self> {
        let char_comma = Char(&',');
        let until_comma = Until(",");
        let until_star = Until("*");
        let mut ctx: std::sync::MutexGuard<'static, StrParserContext<'_>> =
            StrParserContext::new(sentence);
        ctx.skip_strict::<Until, String>(&until_comma)?
            .skip_strict(&char_comma)?;
        let utc_time: Option<chrono::DateTime<chrono::Utc>> = ctx.take(&NmeaUtc());
        let day = ctx.skip_strict(&char_comma)?.take(&until_comma);
        let month = ctx.skip_strict(&char_comma)?.take(&until_comma);
        let year = ctx.skip_strict(&char_comma)?.take(&until_comma);
        let local_zone_description = ctx.skip_strict(&char_comma)?.take(&until_comma);
        let local_zone_minutes_description = ctx.skip_strict(&char_comma)?.take(&until_star);

        Ok(Zda {
            navigation_system,
            utc_time,
            day,
            month,
            year,
            local_zone_description,
            local_zone_minutes_description,
        })
    }
}

#[cfg(test)]
mod test {
    use test_utils::init_log;

    use super::*;
    #[test]
    fn test_new_zda() -> miette::Result<()> {
        init_log();
        let s = "$GPZDA,160012.71,11,03,2004,-1,00*7D";
        let zda = Zda::new(s, NavigationSystem::GN)?;
        println!("{:?}", zda);
        Ok(())
    }
}
