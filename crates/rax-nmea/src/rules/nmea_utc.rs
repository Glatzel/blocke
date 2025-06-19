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
    fn apply(&self, input: &'a str) -> Option<(DateTime<Utc>, &'a str)> {
        // Log the input at trace level.
        clerk::trace!("NmeaUtc rule: input='{}'", input);

        // Find the first comma, which separates the UTC time from the rest.
        if let Some(first_comma_idx) = input.find(",") {
            let res = &input[..first_comma_idx];
            clerk::debug!("utc hhmmss: {}", res);

            // Split the time into main part and fractional seconds.
            match res.split_once('.') {
                Some((main, frac_sec_str)) => {
                    let frac_str = format!("0.{}", frac_sec_str);
                    // Parse hours, minutes, seconds, and fractional seconds.
                    match (
                        main[0..2].parse::<u32>(),
                        main[2..4].parse::<u32>(),
                        main[4..6].parse::<u32>(),
                        frac_str.parse::<f64>(),
                    ) {
                        (Ok(hour), Ok(min), Ok(sec), Ok(frac_sec)) => {
                            // Convert fractional seconds to nanoseconds.
                            let nanos = (frac_sec * 1_000_000_000.0).round() as u32;

                            // Build NaiveTime from parsed components.
                            let time = NaiveTime::from_hms_nano_opt(hour, min, sec, nanos)
                                .expect("from_hms_nano_opt");
                            clerk::debug!("NmeaUtc: parsed time: {}", time);

                            // Use today's date for the DateTime.
                            let today = Utc::now().date_naive();
                            let dt =
                                NaiveDate::from_ymd_opt(today.year(), today.month(), today.day())
                                    .expect("Error from_ymd_opt")
                                    .and_time(time);
                            clerk::debug!("NmeaUtc: constructed DateTime<Utc>: {}", dt);

                            Some((dt.and_utc(), &input[first_comma_idx..]))
                        }
                        _ => {
                            clerk::warn!("NmeaUtc: failed to parse time components from '{}'", res);
                            None
                        }
                    }
                }
                None => {
                    match (
                        res[0..2].parse::<u32>(),
                        res[2..4].parse::<u32>(),
                        res[4..6].parse::<u32>(),
                    ) {
                        (Ok(hour), Ok(min), Ok(sec)) => {
                            // Build NaiveTime from parsed components.
                            let time = NaiveTime::from_hms_nano_opt(hour, min, sec, 0)
                                .expect("from_hms_nano_opt");
                            clerk::debug!("NmeaUtc: parsed time: {}", time);

                            // Use today's date for the DateTime.
                            let today = Utc::now().date_naive();
                            let dt =
                                NaiveDate::from_ymd_opt(today.year(), today.month(), today.day())
                                    .expect("Error from_ymd_opt")
                                    .and_time(time);
                            clerk::debug!("NmeaUtc: constructed DateTime<Utc>: {}", dt);

                            Some((dt.and_utc(), &input[first_comma_idx..]))
                        }
                        _ => {
                            clerk::warn!("NmeaUtc: failed to parse time components from '{}'", res);
                            None
                        }
                    }
                }
            }
        } else {
            clerk::warn!("NmeaUtc: input does not contain a comma: '{}'", input);
            None
        }
    }
}
