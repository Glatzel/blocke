use std::fmt::Debug;
use std::str::FromStr;

use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use miette::{Context, IntoDiagnostic};

macro_rules! readonly_struct {
    ($name:ident, $($struct_doc:expr)+, $({$field:ident: $type:ty $(, $field_doc:expr)?}),*) => {
        $(#[doc=$struct_doc])+
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
        pub struct $name {
            $( $field: $type ),*
        }

        impl $name {
            // Getter methods for each field
            $(
                $(#[doc=$field_doc])?
                pub fn $field(&self) -> &$type {
                    &self.$field
                }
            )*
        }
    }
}

pub(crate) use readonly_struct;

pub(crate) fn get_sentense_parts<'a>(sentense: &'a str) -> Vec<&'a str> {
    let parts: Vec<&str> = sentense
        .split("*")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split(',')
        .collect();
    parts
}
pub(crate) fn parse_utc(sentense: &[&str], index: usize) -> miette::Result<Option<DateTime<Utc>>> {
    if let Some(hhmmss) = sentense.get(index) {
        let (main, frac_sec_str) = hhmmss.split_once('.').unwrap_or((hhmmss, "0"));
        clerk::debug!("utc hhmmss: {}", hhmmss);

        let hour = main[0..2].parse::<u32>().into_diagnostic()?;
        let min = main[2..4].parse::<u32>().into_diagnostic()?;
        let sec = main[4..6].parse::<u32>().into_diagnostic()?;

        // Convert fraction to nanoseconds
        let frac_str = format!("0.{}", frac_sec_str);
        let frac_sec = frac_str.parse::<f64>().into_diagnostic()?;
        let nanos = (frac_sec * 1_000_000_000.0).round() as u32;

        let time = NaiveTime::from_hms_nano_opt(hour, min, sec, nanos).expect("from_hms_nano_opt");
        clerk::debug!("time: {}", time);
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
        clerk::warn!(
            "At least one of longitude digits or hemi-sphere is none.lon_index: {}, hemi_index: {}",
            lat_index,
            hemi_index
        );
        return Ok(None);
    };
    if ddmm.len() < 4 {
        miette::bail!("Invalid latitude Format: {}.", ddmm);
    }

    let (deg_str, min_str) = ddmm.split_at(2);
    let deg = deg_str
        .parse::<f64>()
        .into_diagnostic()
        .wrap_err(format!("deg: {}", deg_str))?;
    let min = min_str
        .parse::<f64>()
        .into_diagnostic()
        .wrap_err(format!("min: {}", min_str))?; // mm.mmmm

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
        clerk::warn!(
            "At least one of longitude digits or hemi-sphere is none.lon_index: {}, hemi_index: {}",
            lon_index,
            hemi_index
        );
        return Ok(None);
    };

    if dddmm.len() < 5 {
        miette::bail!("Invalid longitude format: {}", dddmm);
    }

    let (deg_str, min_str) = dddmm.split_at(3);
    let deg = deg_str
        .parse::<f64>()
        .into_diagnostic()
        .wrap_err(format!("deg: {}", deg_str))?;
    let min = min_str
        .parse::<f64>()
        .into_diagnostic()
        .wrap_err(format!("min: {}", min_str))?; // mm.mmmm?;
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
        clerk::warn!("Empty string, index: {}", index);
        return Ok(None);
    };
    if s.is_empty() {
        clerk::warn!("Empty string, index: {}", index);
        return Ok(None);
    }
    Ok(Some(s.parse::<T>().map_err(|e| {
        miette::miette!(
            "{:?}: {} : {}, index: {}",
            e,
            std::any::type_name::<T>(),
            s,
            index
        )
    })?))
}
pub(crate) fn is_valid(sentence: &str) -> bool {
    if !sentence.starts_with('$') {
        clerk::warn!("sentence doesn't start with `$`");
        return false;
    }

    let Some(star_pos) = sentence.find('*') else {
        clerk::warn!("Missing checksum delimiter `*`");
        return false;
    };

    let (data, checksum_str) = sentence[1..].split_at(star_pos - 1); // skip $
    let checksum_str = &checksum_str[1..];
    clerk::debug!("data: `{}`,checksum_str: `{}`", data, checksum_str);

    if checksum_str.len() != 2 {
        clerk::warn!("require checksum_str lenth 2, get {}", checksum_str.len());
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

#[cfg(test)]
mod test {
    use float_cmp::assert_approx_eq;
    use test_utils::init_log;

    use super::*;
    #[test]
    fn test_parse_hhmmss_fractional() -> miette::Result<()> {
        init_log();
        let inputs = ["235959", "235959.1", "235959.12", "235959.123456789"];
        for i in 0..inputs.len() {
            let utc = parse_utc(inputs.as_slice(), i)?;
            println!("{} -> {:?}", inputs[i], utc);
        }
        Ok(())
    }

    #[test]
    fn test_parse_latitude() -> miette::Result<()> {
        init_log();
        // N hemisphere
        let lat = parse_latitude(&["4916.45", "N"], 0, 1)?;
        // 49 deg + 16.45/60 min
        assert_approx_eq!(f64, lat.unwrap(), 49.0 + 16.45 / 60.0, epsilon = 1e-6);

        // S hemisphere
        let lat = parse_latitude(&["4916.45", "S"], 0, 1)?;
        assert_approx_eq!(f64, lat.unwrap(), -(49.0 + 16.45 / 60.0), epsilon = 1e-6);
        Ok(())
    }
    #[test]
    fn test_parse_longitude() -> miette::Result<()> {
        init_log();
        // E hemisphere
        let lat = parse_longitude(&["12345.67", "E"], 0, 1)?;
        // 49 deg + 16.45/60 min
        assert_approx_eq!(f64, lat.unwrap(), 123.0 + 45.67 / 60.0, epsilon = 1e-6);

        // W hemisphere
        let lat = parse_longitude(&["12345.67", "W"], 0, 1)?;
        assert_approx_eq!(f64, lat.unwrap(), -(123.0 + 45.67 / 60.0), epsilon = 1e-6);
        Ok(())
    }
}
