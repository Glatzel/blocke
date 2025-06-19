use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use rax::str_parser::IRule;

pub struct NmeaUtc();
impl IRule for NmeaUtc {
    fn name(&self) -> &str {
        todo!()
    }
}
impl<'a> rax::str_parser::IStrTakeRule<'a, DateTime<Utc>> for NmeaUtc {
    fn apply(&self, input: &'a str) -> Option<(DateTime<Utc>, &'a str)> {
        if let Some(first_comma_idx) = input.find(",") {
            let res = &input[..first_comma_idx];
            clerk::debug!("utc hhmmss: {}", res);
            match res.split_once('.') {
                Some((main, frac_sec_str)) => {
                    let frac_str = format!("0.{}", frac_sec_str);
                    match (
                        main[0..2].parse::<u32>(),
                        main[2..4].parse::<u32>(),
                        main[4..6].parse::<u32>(),
                        frac_str.parse::<f64>(),
                    ) {
                        (Ok(hour), Ok(min), Ok(sec), Ok(frac_sec)) => {
                            let nanos = (frac_sec * 1_000_000_000.0).round() as u32;

                            let time = NaiveTime::from_hms_nano_opt(hour, min, sec, nanos)
                                .expect("from_hms_nano_opt");
                            clerk::debug!(
                                "time:
                    {}",
                                time
                            );
                            let today = Utc::now().date_naive();
                            let dt =
                                NaiveDate::from_ymd_opt(today.year(), today.month(), today.day())
                                    .expect("Error from_ymd_opt")
                                    .and_time(time);
                            Some((dt.and_utc(), &input[first_comma_idx..]))
                        }
                        _ => None,
                    }
                }
                None => None,
            }
        } else {
            None
        }
    }
}
