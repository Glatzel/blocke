use crate::utils::readonly_struct;
use crate::{FaaMode, NavigationSystem};
use crate::utils::*;
readonly_struct!(
    Vtg ,
    "",
    {navigation_system: NavigationSystem},
    {is_valid: bool},

    {course_over_ground_true: Option<f64>},
    {course_over_ground_magnetic : Option<f64>},
    {speed_over_ground_knots: Option<f64>},
    {speed_over_ground_kph: Option<f64>},
    {mode: Option<FaaMode>}
);

    pub fn new_vtg(sentence: &str) -> miette::Result<Vtg> {
        let parts: Vec<&str> = Self::get_sentense_parts(sentence);
        Ok(Vtg::new(
            Self::get_navigation_system(&sentence)?,
            Self::is_valid(sentence),
            Self::parse_primitive(&parts, 1)?,
            Self::parse_primitive(&parts, 3)?,
            Self::parse_primitive(&parts, 5)?,
            Self::parse_primitive(&parts, 7)?,
            Self::parse_primitive(&parts, 9)?,
        ))
    }

#[cfg(test)]
mod test {
    use test_utils::init_log;

  
    #[test]
    fn test_new_vtg() -> miette::Result<()> {
        init_log();
        let s = "$GPVTG,220.86,T,,M,2.550,N,4.724,K,A*34";
        for (i, v) in get_sentense_parts(s).iter().enumerate() {
            println!("{i}:{v}");
        }
        let vtg = new_vtg(s)?;
        println!("{:?}", vtg);
        assert!(vtg.is_valid);
        Ok(())
    }
}
