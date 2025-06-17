use crate::INmeaData;
use crate::nmea_data::NavigationSystem;
use crate::utils::{readonly_struct, *};
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
impl INmeaData for Zda {
    fn parse_sentense(sentence: &str, navigation_system: NavigationSystem) -> miette::Result<Zda> {
        let parts: Vec<&str> = get_sentense_parts(sentence);
        Ok(Zda {
            navigation_system,
            is_valid: is_valid(sentence),
            utc_time: parse_utc(&parts, 1)?,
            day: parse_primitive(&parts, 2)?,
            month: parse_primitive(&parts, 3)?,
            year: parse_primitive(&parts, 4)?,
            local_zone_description: parse_primitive(&parts, 5)?,
            local_zone_minutes_description: parse_primitive(&parts, 6)?,
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
        for (i, v) in get_sentense_parts(s).iter().enumerate() {
            println!("{i}:{v}");
        }
        let zda = Zda::parse_sentense(s, NavigationSystem::GN)?;
        println!("{:?}", zda);
        assert!(zda.is_valid);
        Ok(())
    }
}
