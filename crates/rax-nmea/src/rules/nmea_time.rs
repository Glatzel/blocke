use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use rax::str_parser::IRule;

use crate::UNTIL_COMMA_DISCARD;

/// Rule to parse an NMEA UTC time string in the format "hhmmss.sss,...".
/// Converts the time to a `DateTime<Utc>` using today's date.
/// Returns a tuple of (DateTime<Utc>, rest_of_input) if successful, otherwise
/// None.
pub struct NmeaTime();

impl IRule for NmeaTime {
    fn name(&self) -> &str { "NmeaUtc" }
}

impl<'a> rax::str_parser::IStrFlowRule<'a> for NmeaTime {
    type Output = NaiveTime;
    /// Applies the NmeaUtc rule to the input string.
    /// Parses the UTC time, converts to `DateTime<Utc>` using today's date, and
    /// returns the result and the rest of the string. Logs each step for
    /// debugging.
    fn apply(&self, input: &'a str) -> (std::option::Option<NaiveTime>, &'a str) {
        clerk::trace!("NmeaUtc rule: input='{}'", input);

        let (res, rest) = UNTIL_COMMA_DISCARD.apply(input);
        match res {
            Some(res) => {
                // Try to split the time into main part and fractional seconds.
                let (main, nanos) = match res.split_once('.') {
                    Some((main, frac_sec_str)) => {
                        let nanos = format!("0.{frac_sec_str}")
                            .parse::<f64>()
                            .map(|f| (f * 1_000_000_000.0).round() as u32)
                            .unwrap_or(0);
                        clerk::debug!("NmeaUtc: parsed fractional seconds: {}", nanos);
                        (main, nanos)
                    }
                    None => (res, 0),
                };

                // Parse hours, minutes, seconds.
                let hour = match main.get(0..2).and_then(|s| s.parse::<u32>().ok()) {
                    Some(h) => h,
                    None => {
                        clerk::warn!(
                            "NmeaUtc: failed to parse hour from '{}',input='{}'",
                            main,
                            input
                        );
                        return (None, rest);
                    }
                };
                let min = match main.get(2..4).and_then(|s| s.parse::<u32>().ok()) {
                    Some(m) => m,
                    None => {
                        clerk::warn!(
                            "NmeaUtc: failed to parse minute from '{}',input='{}'",
                            main,
                            input
                        );
                        return (None, rest);
                    }
                };
                let sec = match main.get(4..).and_then(|s| s.parse::<u32>().ok()) {
                    Some(s) => s,
                    None => {
                        clerk::warn!(
                            "NmeaUtc: failed to parse second from '{}',input='{}'",
                            main,
                            input
                        );
                        return (None, rest);
                    }
                };
                clerk::debug!(
                    "NmeaUtc: parsed hour={}, min={}, sec={}, nanos={}",
                    hour,
                    min,
                    sec,
                    nanos
                );

                // Build NaiveTime from parsed components.
                let time = match NaiveTime::from_hms_nano_opt(hour, min, sec, nanos) {
                    Some(t) => {
                        clerk::debug!("NmeaUtc: parsed time: {}", t);
                        t
                    }
                    None => {
                        clerk::warn!(
                            "NmeaUtc: invalid time: hour={}, min={}, sec={}, nanos={}",
                            hour,
                            min,
                            sec,
                            nanos
                        );
                        return (None, rest);
                    }
                };

                (Some(time), rest)
            }
            None => {
                clerk::info!("NmeaUtc: got empty string.");
                (None, input)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Timelike, Utc};
    use rax::str_parser::IStrFlowRule;

    use super::*;

    #[test]
    fn test_nmea_utc_valid() {
        let rule = NmeaTime();
        let today = Utc::now().date_naive();
        let (dt, rest) = rule.apply("123456.789,foo,bar");
        let dt = dt.expect("Should parse valid UTC time");
        assert_eq!(dt.hour(), 12);
        assert_eq!(dt.minute(), 34);
        assert_eq!(dt.second(), 56);
        assert_eq!(dt.nanosecond(), 789_000_000);

        assert_eq!(rest, "foo,bar");
    }

    #[test]
    fn test_nmea_utc_no_fraction() {
        let rule = NmeaTime();
        let today = Utc::now().date_naive();
        let (dt, rest) = rule.apply("235959,rest");
        let dt = dt.expect("Should parse valid UTC time");
        assert_eq!(dt.hour(), 23);
        assert_eq!(dt.minute(), 59);
        assert_eq!(dt.second(), 59);
        assert_eq!(dt.nanosecond(), 0);

        assert_eq!(rest, "rest");
    }

    #[test]
    fn test_nmea_utc_invalid_hour() {
        let rule = NmeaTime();
        let (dt, rest) = rule.apply("xx3456,foo");
        assert!(dt.is_none());
        assert_eq!(rest, "foo");
    }

    #[test]
    fn test_nmea_utc_invalid_minute() {
        let rule = NmeaTime();
        let (dt, rest) = rule.apply("12xx56,foo");
        assert!(dt.is_none());
        assert_eq!(rest, "foo");
    }

    #[test]
    fn test_nmea_utc_invalid_second() {
        let rule = NmeaTime();
        let (dt, rest) = rule.apply("1234xx,foo");
        assert!(dt.is_none());
        assert_eq!(rest, "foo");
    }

    #[test]
    fn test_nmea_utc_empty() {
        let rule = NmeaTime();
        let (dt, rest) = rule.apply(",foo");
        assert!(dt.is_none());
        assert_eq!(rest, "foo");
    }

    #[test]
    fn test_nmea_utc_no_comma() {
        let rule = NmeaTime();
        let (dt, rest) = rule.apply("123456");
        assert!(dt.is_none());
        assert_eq!(rest, "123456");
    }
}
