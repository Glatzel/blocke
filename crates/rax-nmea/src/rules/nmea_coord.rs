use rax_parser::str_parser::{IRule, IStrFlowRule};

/// Rule to parse an NMEA coordinate in the format "DDDMM.MMM,<sign>,...".
/// Converts the coordinate to decimal degrees, applying the correct sign.
/// Returns a tuple of (decimal_degrees, rest_of_input) if successful, otherwise
/// None.
pub struct NmeaCoord();

impl IRule for NmeaCoord {
    fn name(&self) -> &str { "NmeaCoord" }
}

impl<'a> IStrFlowRule<'a> for NmeaCoord {
    type Output = f64;
    /// Applies the NmeaCoord rule to the input string.
    /// Parses the coordinate and sign, converts to decimal degrees, and returns
    /// the result and the rest of the string. Logs each step for debugging.
    fn apply(&self, input: &'a str) -> (std::option::Option<f64>, &'a str) {
        // Log the input at trace level.
        clerk::trace!("NmeaCoord rule: input='{}'", input);

        // Find the index of the second comma, which separates the sign and the rest.
        if let Some(second_comma_idx) = input
            .char_indices()
            .filter(|&(_, c)| c == ',')
            .nth(1) // 0-based: 0 is first, 1 is second
            .map(|(idx, _)| idx)
        {
            let res = &input[..second_comma_idx];
            // Split into number and sign.
            let (num, sign) = res.split_once(",").unwrap();
            clerk::debug!("NmeaCoord: parsed num='{}', sign='{}'", num, sign);
            match (num.parse::<f64>(), sign) {
                (Ok(v), "E" | "N") => {
                    // Convert to decimal degrees.
                    let deg = (v / 100.0).floor();
                    let min = v - deg * 100.0;
                    let result = deg + min / 60.0;
                    clerk::debug!(
                        "NmeaCoord: positive sign, deg={}, min={}, result={}",
                        deg,
                        min,
                        result
                    );
                    (Some(result), &input[second_comma_idx..])
                }
                (Ok(v), "W" | "S") => {
                    // Convert to negative decimal degrees.
                    let deg = (v / 100.0).floor();
                    let min = v - deg * 100.0;
                    let result = -(deg + min / 60.0);
                    clerk::debug!(
                        "NmeaCoord: negative sign, deg={}, min={}, result={}",
                        deg,
                        min,
                        result
                    );
                    (Some(result), &input[second_comma_idx..])
                }
                _ => {
                    // Log parse failure or invalid sign.
                    clerk::info!(
                        "NmeaCoord: failed to parse number or invalid sign: num='{}', sign='{}'",
                        num,
                        sign
                    );
                    (None, &input[second_comma_idx..])
                }
            }
        } else {
            // Log if the input does not contain two commas.
            clerk::warn!("NmeaCoord: input does not contain two commas: '{}'", input);
            (None, input)
        }
    }
}

#[cfg(test)]
mod tests {

    use clerk::tracing::level_filters::LevelFilter;
    use clerk::init_log_with_level;

    use super::*;

    #[test]
    fn test_nmea_coord_east() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = NmeaCoord();
        // 12319.123,E,rest
        let input = "12319.123,E,rest";
        let (val, rest) = rule.apply(input);
        // 12319.123 means 123 degrees, 19.123 minutes
        // deg = 123, min = 19.123, value = 123 + 19.123/60
        let expected = 123.0 + 19.123 / 60.0;
        assert_eq!(val, Some(expected));
        assert_eq!(rest, ",rest");
    }

    #[test]
    fn test_nmea_coord_west() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = NmeaCoord();
        let input = "12319.123,W,foo";

        let (val, rest) = rule.apply(input);
        let expected = -(123.0 + 19.123 / 60.0);
        assert_eq!(val, Some(expected));
        assert_eq!(rest, ",foo");
    }

    #[test]
    fn test_nmea_coord_north() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = NmeaCoord();
        let input = "4807.038,N,bar";

        let (val, rest) = rule.apply(input);
        let expected = 48.0 + 7.038 / 60.0;
        float_cmp::assert_approx_eq!(f64, val.unwrap(), expected);
        assert_eq!(rest, ",bar");
    }

    #[test]
    fn test_nmea_coord_south() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = NmeaCoord();
        let input = "4807.038,S,xyz";

        let (val, rest) = rule.apply(input);
        let expected = -(48.0 + 7.038 / 60.0);
        float_cmp::assert_approx_eq!(f64, val.unwrap(), expected);
        assert_eq!(rest, ",xyz");
    }

    #[test]
    fn test_nmea_coord_invalid_sign() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = NmeaCoord();
        let input = "12319.123,X,rest";

        let (val, rest) = rule.apply(input);
        assert_eq!(val, None);
        assert_eq!(rest, ",rest");
    }

    #[test]
    fn test_nmea_coord_invalid_number() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = NmeaCoord();
        let input = "notanumber,E,rest";

        let (val, rest) = rule.apply(input);
        assert_eq!(val, None);
        assert_eq!(rest, ",rest");
    }

    #[test]
    fn test_nmea_coord_missing_comma() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = NmeaCoord();
        let input = "12319.123Erest";

        let (val, rest) = rule.apply(input);
        assert_eq!(val, None);
        assert_eq!(rest, "12319.123Erest");
    }
}
