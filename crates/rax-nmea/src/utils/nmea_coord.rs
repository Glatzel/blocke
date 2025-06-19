use rax::str_parser::IStrFlowRule;

pub struct NmeaCoord();
impl<'a> IStrFlowRule<'a, f64> for NmeaCoord {
    fn name(&self) -> &str { "NmeaCoord" }

    fn apply(&self, input: &'a str) -> Option<(f64, &'a str)> {
        if let Some(second_comma_idx) = input
            .char_indices()
            .filter(|&(_, c)| c == ',')
            .nth(1) // 0-based: 0 is first, 1 is second
            .map(|(idx, _)| idx)
        {
            let res = &input[..second_comma_idx];
            let (num, sign) = res.split_once(",").unwrap();
            match (num.parse::<f64>(), sign) {
                (Ok(v), "E" | "N") => {
                    let deg = (v / 100.0).floor();
                    let min = v - deg * 100.0;
                    Some((deg + min / 60.0, &input[second_comma_idx..]))
                }
                (Ok(v), "W" | "S") => {
                    let deg = (v / 100.0).floor();
                    let min = v - deg * 100.0;
                    Some((-(deg + min / 60.0), &input[second_comma_idx..]))
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nmea_coord_east() {
        let rule = NmeaCoord();
        // 12319.123,E,rest
        let input = "12319.123,E,rest";
        let result = rule.apply(input);
        // 12319.123 means 123 degrees, 19.123 minutes
        // deg = 123, min = 19.123, value = 123 + 19.123/60
        let expected = 123.0 + 19.123 / 60.0;
        assert!(result.is_some());
        let (val, rest) = result.unwrap();
        assert!((val - expected).abs() < 1e-6);
        assert_eq!(rest, ",rest");
    }

    #[test]
    fn test_nmea_coord_west() {
        let rule = NmeaCoord();
        let input = "12319.123,W,foo";
        let result = rule.apply(input);
        let expected = -(123.0 + 19.123 / 60.0);
        assert!(result.is_some());
        let (val, rest) = result.unwrap();
        assert!((val - expected).abs() < 1e-6);
        assert_eq!(rest, ",foo");
    }

    #[test]
    fn test_nmea_coord_north() {
        let rule = NmeaCoord();
        let input = "4807.038,N,bar";
        let result = rule.apply(input);
        let expected = 48.0 + 7.038 / 60.0;
        assert!(result.is_some());
        let (val, rest) = result.unwrap();
        assert!((val - expected).abs() < 1e-6);
        assert_eq!(rest, ",bar");
    }

    #[test]
    fn test_nmea_coord_south() {
        let rule = NmeaCoord();
        let input = "4807.038,S,xyz";
        let result = rule.apply(input);
        let expected = -(48.0 + 7.038 / 60.0);
        assert!(result.is_some());
        let (val, rest) = result.unwrap();
        assert!((val - expected).abs() < 1e-6);
        assert_eq!(rest, ",xyz");
    }

    #[test]
    fn test_nmea_coord_invalid_sign() {
        let rule = NmeaCoord();
        let input = "12319.123,X,rest";
        let result = rule.apply(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_nmea_coord_invalid_number() {
        let rule = NmeaCoord();
        let input = "notanumber,E,rest";
        let result = rule.apply(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_nmea_coord_missing_comma() {
        let rule = NmeaCoord();
        let input = "12319.123Erest";
        let result = rule.apply(input);
        assert!(result.is_none());
    }
}
