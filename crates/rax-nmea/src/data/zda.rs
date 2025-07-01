use rax_parser::str_parser::rules::{Char, Until};
use rax_parser::str_parser::{ParseOptExt, StrParserContext};

use crate::NmeaUtc;
use crate::data::Talker;
use crate::macros::readonly_struct;

readonly_struct!(
    Zda ,
    "Zda",
    {navigation_system: Talker},

    {utc_time: Option<chrono::DateTime<chrono::Utc>>},
    {day : Option<u8>},
    {month: Option<u8>},
    {year: Option<u16>},
    {local_zone_description: Option<i8>},
    {local_zone_minutes_description: Option<u8>}
);

impl Zda {
    pub fn new(ctx: &mut StrParserContext, navigation_system: Talker) -> miette::Result<Self> {
        let char_comma = Char(&',');
        let until_comma = Until(",");
        let until_star = Until("*");

        let utc_time = ctx
            .skip_strict(&until_comma)?
            .skip_strict(&char_comma)?
            .take(&NmeaUtc());
        let day = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let month = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let year = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let local_zone_description = ctx.skip_strict(&char_comma)?.take(&until_comma).parse_opt();
        let local_zone_minutes_description =
            ctx.skip_strict(&char_comma)?.take(&until_star).parse_opt();

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

use std::fmt;

impl fmt::Debug for Zda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("Zda");
        ds.field("navigation_system", &self.navigation_system);

        if let Some(ref utc_time) = self.utc_time {
            ds.field("utc_time", utc_time);
        }
        if let Some(day) = self.day {
            ds.field("day", &day);
        }
        if let Some(month) = self.month {
            ds.field("month", &month);
        }
        if let Some(year) = self.year {
            ds.field("year", &year);
        }
        if let Some(local_zone_description) = self.local_zone_description {
            ds.field("local_zone_description", &local_zone_description);
        }
        if let Some(local_zone_minutes_description) = self.local_zone_minutes_description {
            ds.field(
                "local_zone_minutes_description",
                &local_zone_minutes_description,
            );
        }

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
        let s = "$GPZDA,160012.71,11,03,2004,-1,00*7D";
        let mut ctx = StrParserContext::new();
        let zda = Zda::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", zda);
        Ok(())
    }
}
