use chrono::NaiveDate;
use rax::str_parser::IRule;

pub struct NmeaDate();

impl IRule for NmeaDate {
    fn name(&self) -> &str { "NmeaDate" }
}

impl<'a> rax::str_parser::IStrFlowRule<'a, NaiveDate> for NmeaDate {
    /// Applies the NmeaUtc rule to the input string.
    /// Parses the UTC time, converts to `DateTime<Utc>` using today's date, and
    /// returns the result and the rest of the string. Logs each step for
    /// debugging.
    fn apply(&self, input: &'a str) -> Option<(NaiveDate, &'a str)> {
        clerk::trace!("NmeaUtc rule: input='{}'", input);

        // Find the first comma, which separates the UTC time from the rest.
        let first_comma_idx = input.find(",")?;
        let res = &input[..first_comma_idx];
        clerk::debug!("utc ddmmyy: {}", res);

        let (day, month, year) = (
            res.get(0..2)?.parse::<u32>().ok()?,
            res.get(2..4)?.parse::<u32>().ok()?,
            res.get(4..6)?.parse::<i32>().ok()?,
        );
        let dt = NaiveDate::from_ymd_opt(year + 2000, month, day)?;
        Some((dt, &input[first_comma_idx..]))
    }
}
