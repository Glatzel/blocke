use crate::INmeaData;
use crate::nmea_data::NavigationSystem;
use crate::utils::*;
readonly_struct!(
    Dhv ,
    "Dhv",
    {navigation_system: NavigationSystem},
    {is_valid: bool},

    {utc_time: Option<chrono::DateTime<chrono::Utc>>},
    {speed3d : Option<f64>},
    {speed_x: Option<f64>},
    {speed_y: Option<f64>},
    {speed_z: Option<f64>},
    {gdspd: Option<f64>}
);
impl INmeaData for Dhv {
    fn parse_sentence(sentence: &str, navigation_system: NavigationSystem) -> miette::Result<Dhv> {
        let parts: Vec<&str> = get_sentence_parts(sentence);
        Ok(Dhv {
            navigation_system,
            is_valid: is_valid(sentence),
            utc_time: parse_utc(&parts, 1)?,
            speed3d: parse_primitive(&parts, 2)?,
            speed_x: parse_primitive(&parts, 3)?,
            speed_y: parse_primitive(&parts, 4)?,
            speed_z: parse_primitive(&parts, 5)?,
            gdspd: parse_primitive(&parts, 6)?,
        })
    }
}

#[cfg(test)]
mod test {
    use test_utils::init_log;

    use super::*;
    #[test]
    fn test_new_dhv() -> miette::Result<()> {
        init_log();
        let s = "$GNDHV,021150.000,0.03,0.006,-0.042,-0.026,0.06*65";
        for (i, v) in get_sentence_parts(s).iter().enumerate() {
            println!("{i}:{v}");
        }
        let dhv = Dhv::parse_sentence(s, NavigationSystem::GN)?;
        println!("{:?}", dhv);
        assert!(dhv.is_valid);
        Ok(())
    }
}
