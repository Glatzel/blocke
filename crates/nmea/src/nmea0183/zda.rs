use crate::NavigationSystem;
use crate::utils::readonly_struct;

readonly_struct!(
    Zda ,
    "",
    {navigation_system: NavigationSystem},
    {is_valid: bool},

    {utc_time: Option<chrono::DateTime<chrono::Utc>>},
    {day : Option<u8>},
    {month: Option<u8>},
    {year: Option<u16>},
    {local_zone_description: Option<i8>},
    {local_zone_minutes_description: Option<u8>}
);
impl crate::parser::NmeaParser {
    pub fn new_zda(sentence: &str) -> miette::Result<Zda> {
        let parts: Vec<&str> = Self::get_sentense_parts(sentence);
        Ok(Zda::new(
            Self::get_navigation_system(&sentence)?,
            Self::is_valid(sentence),
            Self::parse_utc(&parts, 1)?,
            Self::parse_primitive(&parts, 2)?,
            Self::parse_primitive(&parts, 3)?,
            Self::parse_primitive(&parts, 4)?,
            Self::parse_primitive(&parts, 5)?,
            Self::parse_primitive(&parts, 6)?,
        ))
    }
}
#[cfg(test)]
mod test {
    use test_utils::init_log;

    use crate::parser::NmeaParser;
    #[test]
    fn test_new_zda() -> miette::Result<()> {
        init_log();
        let s = "$GPZDA,160012.71,11,03,2004,-1,00*7D";
        for (i, v) in NmeaParser::get_sentense_parts(s).iter().enumerate() {
            println!("{i}:{v}");
        }
        let zda = NmeaParser::new_zda(s)?;
        println!("{:?}", zda);
        assert!(zda.is_valid);
        Ok(())
    }
}
