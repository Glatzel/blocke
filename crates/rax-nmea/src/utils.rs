mod nmea_coord;
mod validate_nmea;
use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use miette::{Context, IntoDiagnostic};
pub use nmea_coord::*;
pub use validate_nmea::*;

pub(crate) fn get_sentence_parts(sentence: &str) -> Vec<&str> {
    let parts: Vec<&str> = sentence
        .split("*")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split(',')
        .collect();
    parts
}
pub(crate) fn parse_utc(sentence: &[&str], index: usize) -> miette::Result<Option<DateTime<Utc>>> {
    if let Some(hhmmss) = sentence.get(index) {
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
    sentence: &[&str],
    lat_index: usize,
    hemi_index: usize,
) -> miette::Result<Option<f64>> {
    let (Some(ddmm), Some(hemi)) = (sentence.get(lat_index), sentence.get(hemi_index)) else {
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
    sentence: &[&str],
    lon_index: usize,
    hemi_index: usize,
) -> miette::Result<Option<f64>> {
    let (Some(dddmm), Some(hemi)) = (sentence.get(lon_index), sentence.get(hemi_index)) else {
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
