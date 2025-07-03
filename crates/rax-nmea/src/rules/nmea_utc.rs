use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use rax_parser::str_parser::IRule;

/// Rule to parse an NMEA UTC time string in the format "hhmmss.sss,...".
/// Converts the time to a `DateTime<Utc>` using today's date.
/// Returns a tuple of (DateTime<Utc>, rest_of_input) if successful, otherwise
/// None.
pub struct NmeaUtc();

impl IRule for NmeaUtc {
    fn name(&self) -> &str { "NmeaUtc" }
}

impl<'a> rax_parser::str_parser::IStrFlowRule<'a> for NmeaUtc {
    type Output = DateTime<Utc>;
    /// Applies the NmeaUtc rule to the input string.
    /// Parses the UTC time, converts to `DateTime<Utc>` using today's date, and
    /// returns the result and the rest of the string. Logs each step for
    /// debugging.
    fn apply(&self, input: &'a str) -> (std::option::Option<DateTime<Utc>>, &'a str) {
        clerk::trace!("NmeaUtc rule: input='{}'", input);

        // Find the first comma, which separates the UTC time from the rest.
        let first_comma_idx = match input.find(",") {
            Some(idx) => idx,
            None => {
                clerk::warn!("NmeaUtc: no comma found in input '{}'", input);
                return (None, input);
            }
        };
        let res = &input[..first_comma_idx];
        clerk::debug!("utc hhmmss: {}", res);
        if res.is_empty() {
            clerk::info!("NmeaUtc: got empty string.");
            return (None, input);
        }

        // Try to split the time into main part and fractional seconds.
        let (main, nanos) = match res.split_once('.') {
            Some((main, frac_sec_str)) => {
                let nanos = format!("0.{}", frac_sec_str)
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
                clerk::warn!("NmeaUtc: failed to parse hour from '{}'", main);
                return (None, &input[first_comma_idx..]);
            }
        };
        let min = match main.get(2..4).and_then(|s| s.parse::<u32>().ok()) {
            Some(m) => m,
            None => {
                clerk::warn!("NmeaUtc: failed to parse minute from '{}'", main);
                return (None, &input[first_comma_idx..]);
            }
        };
        let sec = match main.get(4..6).and_then(|s| s.parse::<u32>().ok()) {
            Some(s) => s,
            None => {
                clerk::warn!("NmeaUtc: failed to parse second from '{}'", main);
                return (None, &input[first_comma_idx..]);
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
                return (None, &input[first_comma_idx..]);
            }
        };

        // Use today's date for the DateTime.
        let today = Utc::now().date_naive();
        let dt = match NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()) {
            Some(date) => {
                let dt = date.and_time(time);
                clerk::debug!("NmeaUtc: constructed DateTime<Utc>: {}", dt);
                dt
            }
            None => {
                clerk::warn!(
                    "NmeaUtc: invalid date: y={}, m={}, d={}",
                    today.year(),
                    today.month(),
                    today.day()
                );
                return (None, &input[first_comma_idx..]);
            }
        };

        (Some(dt.and_utc()), &input[first_comma_idx..])
    }
}
