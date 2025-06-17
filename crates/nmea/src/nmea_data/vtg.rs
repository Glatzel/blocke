use crate::nmea_data::NavigationSystem;
use crate::utils::{readonly_struct, *};
use crate::{FaaMode, INmeaData};
readonly_struct!(
    Vtg ,
    "Vtg",
    {navigation_system: NavigationSystem},
    {is_valid: bool},

    {course_over_ground_true: Option<f64>},
    {course_over_ground_magnetic : Option<f64>},
    {speed_over_ground_knots: Option<f64>},
    {speed_over_ground_kph: Option<f64>},
    {mode: Option<FaaMode>}
);
impl INmeaData for Vtg {
    fn parse_sentense(sentence: &str, navigation_system: NavigationSystem) -> miette::Result<Vtg> {
        let parts: Vec<&str> = get_sentense_parts(sentence);
        Ok(Vtg {
            navigation_system,
            is_valid: is_valid(sentence),
            course_over_ground_true: parse_primitive(&parts, 1)?,
            course_over_ground_magnetic: parse_primitive(&parts, 3)?,
            speed_over_ground_knots: parse_primitive(&parts, 5)?,
            speed_over_ground_kph: parse_primitive(&parts, 7)?,
            mode: parse_primitive(&parts, 9)?,
        })
    }
}

#[cfg(test)]
mod test {
    use test_utils::init_log;

    use super::*;
    #[test]
    fn test_new_vtg() -> miette::Result<()> {
        init_log();
        let s = "$GPVTG,220.86,T,,M,2.550,N,4.724,K,A*34";
        for (i, v) in get_sentense_parts(s).iter().enumerate() {
            println!("{i}:{v}");
        }
        let vtg = Vtg::parse_sentense(s, NavigationSystem::GN)?;
        println!("{:?}", vtg);
        assert!(vtg.is_valid);
        Ok(())
    }
}
