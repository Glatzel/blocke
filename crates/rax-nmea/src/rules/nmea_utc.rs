use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use rax::str_parser::IRule;

/// Rule to parse an NMEA UTC time string in the format "hhmmss.sss,...".
/// Converts the time to a `DateTime<Utc>` using today's date.
/// Returns a tuple of (DateTime<Utc>, rest_of_input) if successful, otherwise
/// None.
pub struct NmeaUtc();

impl IRule for NmeaUtc {
    fn name(&self) -> &str { "NmeaUtc" }
}

impl<'a> rax::str_parser::IStrFlowRule<'a, DateTime<Utc>> for NmeaUtc {
    /// Applies the NmeaUtc rule to the input string.
    /// Parses the UTC time, converts to `DateTime<Utc>` using today's date, and
    /// returns the result and the rest of the string. Logs each step for
    /// debugging.
    fn apply(&self, input: &'a str) -> (std::option::Option<DateTime<Utc>>, &'a str) {
        clerk::trace!("NmeaUtc rule: input='{}'", input);

        // Find the first comma, which separates the UTC time from the rest.
        let first_comma_idx = match input.find(",") {
            Some(idx) => idx,
            None => return (None, input),
        };
        let res = &input[..first_comma_idx];
        clerk::debug!("utc hhmmss: {}", res);

        // Try to split the time into main part and fractional seconds.
        let (main, nanos) = match res.split_once('.') {
            Some((main, frac_sec_str)) => {
                let nanos = format!("0.{}", frac_sec_str)
                    .parse::<f64>()
                    .map(|f| (f * 1_000_000_000.0).round() as u32)
                    .unwrap_or(0);
                (main, nanos)
            }
            None => (res, 0),
        };

        // Parse hours, minutes, seconds.
        let hour = match main.get(0..2).and_then(|s| s.parse::<u32>().ok()) {
            Some(h) => h,
            None => return (None, input),
        };
        let min = match main.get(2..4).and_then(|s| s.parse::<u32>().ok()) {
            Some(m) => m,
            None => return (None, input),
        };
        let sec = match main.get(4..6).and_then(|s| s.parse::<u32>().ok()) {
            Some(s) => s,
            None => return (None, input),
        };
        clerk::debug!(
            "NmeaUtc: parsed hour={}, min={}, sec={}, nanos={}",
            hour,
            min,
            sec,
            nanos
        );

        // Build NaiveTime from parsed components.
        let time = NaiveTime::from_hms_nano_opt(hour, min, sec, nanos)?;
        clerk::debug!("NmeaUtc: parsed time: {}", time);

        // Use today's date for the DateTime.
        let today = Utc::now().date_naive();
        let dt = NaiveDate::from_ymd_opt(today.year(), today.month(), today.day())?.and_time(time);
        clerk::debug!("NmeaUtc: constructed DateTime<Utc>: {}", dt);

        (Some(dt.and_utc()), &input[first_comma_idx..])
    }
}
