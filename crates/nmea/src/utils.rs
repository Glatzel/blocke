use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use miette::IntoDiagnostic;

pub(crate) fn parse_hhmmss_fractional(hhmmss: &str) -> miette::Result<DateTime<Utc>> {
    let (main, frac_sec_str) = hhmmss.split_once('.').unwrap_or((hhmmss, "0")); // if no '.', default to 0

    if main.len() != 6 {
        return Ok(Utc::now());
    }

    let hour = main[0..2].parse::<u32>().into_diagnostic()?;
    let min = main[2..4].parse::<u32>().into_diagnostic()?;
    let sec = main[4..6].parse::<u32>().into_diagnostic()?;
    // Convert fraction to nanoseconds
    let frac_str = format!("0.{}", frac_sec_str);
    let frac_sec = frac_str.parse::<f64>().into_diagnostic()?;
    let nanos = (frac_sec * 1_000_000_000.0).round() as u32;

    let time = NaiveTime::from_hms_nano_opt(hour, min, sec, nanos).expect("from_hms_nano_opt");
    let today = Utc::now().date_naive();
    let dt = NaiveDate::from_ymd_opt(today.year(), today.month(), today.day())
        .expect("Error from_ymd_opt")
        .and_time(time);

    Ok(dt.and_utc())
}
pub(crate) fn parse_latitude(hemi: &str, ddmm: &str) -> miette::Result<f64> {
    if ddmm.len() < 4 {
        miette::bail!("Invalid latitude Format: {}.", ddmm);
    }

    let (deg_str, min_str) = ddmm.split_at(2);
    let deg = deg_str.parse::<f64>().into_diagnostic()?;
    let min = min_str.parse::<f64>().into_diagnostic()?; // mm.mmmm

    let lat = deg + min / 60.0;

    match hemi.to_uppercase().as_str() {
        "N" => Ok(lat),
        "S" => Ok(-lat),
        other => miette::bail!("Unknown hemi: {}.", other),
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_hhmmss_fractional() -> miette::Result<()> {
        let inputs = ["235959", "235959.1", "235959.12", "235959.123456789"];

        for input in inputs {
            let utc = parse_hhmmss_fractional(input)?;
            println!("{} -> {:?}", input, utc);
        }
        Ok(())
    }
}
