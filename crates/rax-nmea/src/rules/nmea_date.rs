use chrono::NaiveDate;
use rax_parser::str_parser::IRule;

pub struct NmeaDate();

impl IRule for NmeaDate {
    fn name(&self) -> &str { "NmeaDate" }
}

impl<'a> rax_parser::str_parser::IStrFlowRule<'a> for NmeaDate {
    type Output = NaiveDate;
    /// Applies the NmeaUtc rule to the input string.
    /// Parses the UTC time, converts to `DateTime<Utc>` using today's date, and
    /// returns the result and the rest of the string. Logs each step for
    /// debugging.
    fn apply(&self, input: &'a str) -> (std::option::Option<NaiveDate>, &'a str) {
        clerk::trace!("NmeaUtc rule: input='{}'", input);

        // Find the first comma, which separates the UTC time from the rest.
        let first_comma_idx = match input.find(",") {
            Some(idx) => idx,
            None => {
                clerk::warn!("NmeaDate: no comma found in input '{}'", input);
                return (None, input);
            }
        };
        let res = &input[..first_comma_idx];
        clerk::debug!("utc ddmmyy: {}", res);

        let day = match res.get(0..2).and_then(|s| s.parse::<u32>().ok()) {
            Some(d) => d,
            None => {
                clerk::info!("NmeaDate: failed to parse day from '{}'", res);
                return (None, &input[first_comma_idx..]);
            }
        };
        let month = match res.get(2..4).and_then(|s| s.parse::<u32>().ok()) {
            Some(m) => m,
            None => {
                clerk::info!("NmeaDate: failed to parse month from '{}'", res);
                return (None, &input[first_comma_idx..]);
            }
        };
        let year = match res.get(4..6).and_then(|s| s.parse::<i32>().ok()) {
            Some(y) => y,
            None => {
                clerk::info!("NmeaDate: failed to parse year from '{}'", res);
                return (None, &input[first_comma_idx..]);
            }
        };
        let dt = match NaiveDate::from_ymd_opt(year + 2000, month, day) {
            Some(date) => {
                clerk::debug!("NmeaDate: parsed date: {}", date);
                date
            }
            None => {
                clerk::warn!(
                    "NmeaDate: invalid date: y={}, m={}, d={}",
                    year + 2000,
                    month,
                    day
                );
                return (None, &input[first_comma_idx..]);
            }
        };
        (Some(dt), &input[first_comma_idx..])
    }
}
