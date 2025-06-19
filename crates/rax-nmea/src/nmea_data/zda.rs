use rax::str_parser::StrParserContext;
use rax::str_parser::flow_rules::{Char, Until};

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
    fn new(sentence: &str, navigation_system: NavigationSystem) -> miette::Result<Zda> {
        let char_comma = Char(&',');
        let until_comma = Until(&",");
        let mut ctx = StrParserContext::new(sentence);
        let utc_time = ctx.skip(&until_comma).take(&NmeaUtc());
        let month = ctx.skip(&char_comma).take(&until_comma);
        let year = ctx.skip(&char_comma).take(&until_comma);
        let local_zone_description = ctx.skip(&char_comma).take(&until_comma);
        let day = ctx.skip(&char_comma).take(&until_comma);
        let local_zone_minutes_description = ctx.skip(&char_comma).take(&until_comma);

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
        for (i, v) in get_sentence_parts(s).iter().enumerate() {
            println!("{i}:{v}");
        }
        let zda = Zda::parse_sentence(s, NavigationSystem::GN)?;
        println!("{:?}", zda);
        assert!(zda.is_valid);
        Ok(())
    }
}
