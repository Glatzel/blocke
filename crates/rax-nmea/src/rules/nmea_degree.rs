use rax::str_parser::{IRule, IStrFlowRule};

use crate::UNTIL_COMMA_DISCARD;

/// Rule to parse an NMEA coordinate in the format "DDDMM.MMM,<sign>,...".
/// Converts the coordinate to decimal degrees, applying the correct sign.
/// Returns a tuple of (decimal_degrees, rest_of_input) if successful, otherwise
/// None.
pub struct NmeaDegree();

impl IRule for NmeaDegree {
    fn name(&self) -> &str { "NmeaDegree" }
}

impl<'a> IStrFlowRule<'a> for NmeaDegree {
    type Output = f64;

    fn apply(&self, input: &'a str) -> (std::option::Option<f64>, &'a str) {
        // Log the input at trace level.
        clerk::trace!("NmeaDegree rule: input='{}'", input);
        let (deg, rest) = UNTIL_COMMA_DISCARD.apply(input);
        let (sign, rest) = UNTIL_COMMA_DISCARD.apply(rest);
        match (deg, sign) {
            (Some(deg), Some(sign)) => {
                clerk::debug!("NmeaDegree: parsed num='{}', sign='{}'", deg, sign);
                match (deg.parse::<f64>(), sign) {
                    (Ok(v), "E" | "N") => (Some(v), rest),
                    (Ok(v), "W" | "S") => (Some(-v), rest),
                    _ => {
                        clerk::info!("NmeaDegree: failed to parse coordinate '{:?}'", (deg, sign));
                        (None, rest)
                    }
                }
            }
            (_, _) => {
                clerk::warn!("NmeaDegree: no second comma found in input '{}'", input);
                (None, input)
            }
        }
    }
}
#[cfg(test)]
mod test {
    use clerk::init_log_with_level;
    use float_cmp::assert_approx_eq;
    use tracing_subscriber::filter::LevelFilter;

    use super::*;
    #[test]
    fn test_nmea_degree() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = NmeaDegree();
        let input = "123.45,N,other_data";
        let (result, rest) = rule.apply(input);
        assert!(result.is_some());
        assert_approx_eq!(f64, result.unwrap(), 123.45);
        assert_eq!(rest, "other_data");
    }
    #[test]
    fn test_nmea_degree_negative() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = NmeaDegree();
        let input = "123.45,S,other_data";
        let (result, rest) = rule.apply(input);
        assert!(result.is_some());
        assert_approx_eq!(f64, result.unwrap(), -123.45);
        assert_eq!(rest, "other_data");
    }
    #[test]
    fn test_nmea_degree_invalid() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = NmeaDegree();
        let input = "invalid_input";
        let (result, rest) = rule.apply(input);
        assert!(result.is_none());
        assert_eq!(rest, input);
    }
    #[test]
    fn test_nmea_degree_no_second_comma() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = NmeaDegree();
        let input = "12345.6789,Nother_data";
        let (result, rest) = rule.apply(input);
        assert!(result.is_none());
        assert_eq!(rest, input);
    }
    #[test]
    fn test_nmea_degree_null() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = NmeaDegree();
        let input = ",,Nother_data";
        let (result, rest) = rule.apply(input);
        assert!(result.is_none());
        assert_eq!(rest, "Nother_data");
    }
}
