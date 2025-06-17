use std::{fmt::Debug, str::FromStr};

use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use miette::IntoDiagnostic;

use crate::primitives::NavigationSystem;
pub struct NmeaParser {}
impl NmeaParser {
    pub(crate) fn get_navigation_system(sentense: &str) -> miette::Result<NavigationSystem> {
        if sentense.len() < 6 {
            miette::bail!("Invalid sentense: {}", sentense);
        }
        NavigationSystem::from_str(&sentense[1,3]).into_diagnostic()
    }
    pub(crate) fn parse_utc(
        sentense: &[&str],
        index: usize,
    ) -> miette::Result<Option<DateTime<Utc>>> {
        if let Some(hhmmss) = sentense.get(index) {
            let (main, frac_sec_str) = hhmmss.split_once('.').unwrap_or((hhmmss, "0"));

            let hour = main[0..2].parse::<u32>().into_diagnostic()?;
            let min = main[2..4].parse::<u32>().into_diagnostic()?;
            let sec = main[4..6].parse::<u32>().into_diagnostic()?;
            // Convert fraction to nanoseconds
            let frac_str = format!("0.{}", frac_sec_str);
            let frac_sec = frac_str.parse::<f64>().into_diagnostic()?;
            let nanos = (frac_sec * 1_000_000_000.0).round() as u32;

            let time =
                NaiveTime::from_hms_nano_opt(hour, min, sec, nanos).expect("from_hms_nano_opt");
            let today = Utc::now().date_naive();
            let dt = NaiveDate::from_ymd_opt(today.year(), today.month(), today.day())
                .expect("Error from_ymd_opt")
                .and_time(time);

            Ok(Some(dt.and_utc()))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn parse_latitude(
        sentense: &[&str],
        lat_index: usize,
        hemi_index: usize,
    ) -> miette::Result<Option<f64>> {
        let (Some(ddmm), Some(hemi)) = (sentense.get(lat_index), sentense.get(hemi_index)) else {
            return Ok(None);
        };
        if ddmm.len() < 4 {
            miette::bail!("Invalid latitude Format: {}.", ddmm);
        }

        let (deg_str, min_str) = ddmm.split_at(2);
        let deg = deg_str.parse::<f64>().into_diagnostic()?;
        let min = min_str.parse::<f64>().into_diagnostic()?; // mm.mmmm

        let lat = deg + min / 60.0;

        match hemi.to_uppercase().as_str() {
            "N" => Ok(Some(lat)),
            "S" => Ok(Some(-lat)),
            other => miette::bail!("Unknown hemi: {}.", other),
        }
    }
    pub(crate) fn parse_longitude(
        sentense: &[&str],
        lon_index: usize,
        hemi_index: usize,
    ) -> miette::Result<Option<f64>> {
        let (Some(dddmm), Some(hemi)) = (sentense.get(lon_index), sentense.get(hemi_index)) else {
            return Ok(None);
        };

        if dddmm.len() < 5 {
            miette::bail!("Invalid longitude format: {}", dddmm);
        }

        let (deg_str, min_str) = dddmm.split_at(3);
        let deg = deg_str.parse::<f64>().into_diagnostic()?;
        let min = min_str.parse::<f64>().into_diagnostic()?;
        let lon = deg + min / 60.0;

        match hemi.to_uppercase().as_str() {
            "E" => Ok(Some(lon)),
            "W" => Ok(Some(-lon)),
            other => miette::bail!("Unknown hemisphere: {}", other),
        }
    }
    pub(crate) fn parse_primitive<T>(sentense: &[&str], index: usize) -> miette::Result<Option<T>>
    where
        T: FromStr,
        T::Err: Debug,
    {
        let Some(s) = sentense.get(index) else {
            return Ok(None);
        };
        if s.is_empty() {
            return Ok(None);
        }
        Ok(Some(
            s.parse::<T>().map_err(|e| miette::miette!("{:?}", e))?,
        ))
    }
    pub(crate) fn is_valid(sentence: &str) -> bool {
        if !sentence.starts_with('$') {
            return false;
        }

        let Some(star_pos) = sentence.find('*') else {
            clerk::warn!("Missing checksum delimiter `*`");
            return false;
        };

        let (data, checksum_str) = sentence[1..].split_at(star_pos - 1); // skip $

        if checksum_str.len() != 2 {
            return false;
        }

        let expected = u8::from_str_radix(checksum_str, 16);
        // .map_err(|_| "Invalid hex checksum".to_string());
        let Ok(expected) = expected else {
            clerk::warn!("Invalid hex checksum");
            return false;
        };

        let calculated = data.bytes().fold(0u8, |acc, b| acc ^ b);

        if calculated != expected {
            clerk::warn!(
                "Checksum mismatch: calculated {:02X}, expected {:02X}",
                calculated,
                expected
            );
            return false;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_parse_hhmmss_fractional() -> miette::Result<()> {
        let inputs = ["235959", "235959.1", "235959.12", "235959.123456789"];

        for i in 0..inputs.len() {
            let utc = NmeaParser::parse_utc(inputs.as_slice(), i)?;
            println!("{} -> {:?}", inputs[i], utc);
        }
        Ok(())
    }

    #[test]
    fn test_parse_latitude() -> miette::Result<()> {
        // N hemisphere
        let lat = NmeaParser::parse_latitude(&["4916.45", "N"], 0, 1)?;
        // 49 deg + 16.45/60 min
        assert_approx_eq!(f64, lat.unwrap(), 49.0 + 16.45 / 60.0, epsilon = 1e-6);

        // S hemisphere
        let lat = NmeaParser::parse_latitude(&["4916.45", "S"], 0, 1)?;
        assert_approx_eq!(f64, lat.unwrap(), -(49.0 + 16.45 / 60.0), epsilon = 1e-6);
        Ok(())
    }
    #[test]
    fn test_parse_longitude() -> miette::Result<()> {
        // E hemisphere
        let lat = NmeaParser::parse_longitude(&["12345.67", "E"], 0, 1)?;
        // 49 deg + 16.45/60 min
        assert_approx_eq!(f64, lat.unwrap(), 123.0 + 45.67 / 60.0, epsilon = 1e-6);

        // W hemisphere
        let lat = NmeaParser::parse_longitude(&["12345.67", "W"], 0, 1)?;
        assert_approx_eq!(f64, lat.unwrap(), -(123.0 + 45.67 / 60.0), epsilon = 1e-6);
        Ok(())
    }
}
