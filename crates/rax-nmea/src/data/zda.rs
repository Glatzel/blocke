use rax_parser::str_parser::{ParseOptExt, StrParserContext};

use crate::data::Talker;
use crate::macros::readonly_struct;
use crate::rules::*;

readonly_struct!(
    Zda ,
    "Zda",
    {talker: Talker},

    {utc_time: Option<chrono::DateTime<chrono::Utc>>},
    {day : Option<u8>},
    {month: Option<u8>},
    {year: Option<u16>},
    {local_zone_description: Option<i8>},
    {local_zone_minutes_description: Option<u8>}
);

impl Zda {
    pub fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;

        let utc_time = ctx
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .take(&NMEA_UTC);
        let day = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let month = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let year = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let local_zone_description = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let local_zone_minutes_description =
            ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_STAR).parse_opt();

        Ok(Zda {
            talker,
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
        let mut ds = f.debug_struct("ZDA");
        ds.field("talker", &self.talker);

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
    use clerk::tracing::level_filters::LevelFilter;
    use clerk::init_log_with_level;
    use super::*;
    #[test]
    fn test_new_zda() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPZDA,160012.71,11,03,2004,-1,00*7D";
        let mut ctx = StrParserContext::new();
        let zda = Zda::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{:?}", zda);
        assert!(zda.utc_time.unwrap().to_string().contains("16:00:12.71"));
        assert_eq!(zda.day.unwrap(), 11);
        assert_eq!(zda.month.unwrap(), 3);
        assert_eq!(zda.year.unwrap(), 2004);
        assert_eq!(zda.local_zone_description.unwrap(), -1);
        assert_eq!(zda.local_zone_minutes_description.unwrap(), 0);
        Ok(())
    }
}
