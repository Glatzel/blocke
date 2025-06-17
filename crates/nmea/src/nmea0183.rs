mod gga;
#[derive(PartialEq, Debug)]
pub enum Nmea0183 {
    GGA,

    Other(String),
}
impl TryFrom<&str> for Nmea0183 {
    type Error = miette::Report;

    fn try_from(sentense: &str) -> miette::Result<Self> {
        let parts: Vec<&str> = sentense.split(",").collect();

        match parts
            .first()
            .expect("Empty string")
            .chars()
            .skip(3)
            .collect::<String>()
            .as_str()
        {
            "GGA" => Ok(Nmea0183::GGA),

            "" => miette::bail!("Empty string."),

            other => Ok(Nmea0183::Other(other.to_string())),
        }
    }
}
#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_nmea0813() -> miette::Result<()> {
        //valid
        assert_eq!(Nmea0183::try_from("$BDGGA")?, Nmea0183::GGA);

        //other
        assert_eq!(
            Nmea0183::try_from("$BDunknown")?,
            Nmea0183::Other("unknown".to_string())
        );

        //invalid
        assert!(Nmea0183::try_from("").is_err());
        assert!(Nmea0183::try_from("$").is_err());

        Ok(())
    }
}
