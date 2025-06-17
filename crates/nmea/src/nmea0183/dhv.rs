use crate::{utils::readonly_struct, NavigationSystem};

readonly_struct!(
    Dhv ,
    "",
    {navigation_system: NavigationSystem},
    {is_valid: bool},

    {utc_time: Option<chrono::DateTime<chrono::Utc>>},
    {speed3d : Option<f64>},
    {speed_x: Option<f64>},
    {speed_y: Option<f64>},
    {speed_z: Option<f64>},
    {gdspd: Option<f64>}
);
impl crate::parser::NmeaParser {
    pub fn new_dhv(sentence: &str) -> miette::Result<Dhv> {
        let parts: Vec<&str> = Self::get_sentense_parts(sentence);
        Ok(Dhv::new(
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
    fn test_new_dhv() -> miette::Result<()> {
        init_log();
        let s = "$GNDHV,021150.000,0.03,0.006,-0.042,-0.026,0.06*65";
        for (i, v) in NmeaParser::get_sentense_parts(s).iter().enumerate() {
            println!("{i}:{v}");
        }
        let dhv = NmeaParser::new_dhv(s)?;
        println!("{:?}", dhv);
        assert!(dhv.is_valid);
        Ok(())
    }
}
