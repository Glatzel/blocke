use std::fmt::Debug;

use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::data::Talker;
use crate::macros::readonly_struct;
use crate::{CHAR_COMMA, NMEA_UTC, UNTIL_COMMA, UNTIL_STAR};

readonly_struct!(
    Gbs,
    "GPS Satellite Fault Detection"
    "# References"
    "* <https://gpsd.gitlab.io/gpsd/NMEA.html#_gbs_gps_satellite_fault_detection>"
    ,
   {talker: Talker},

   {
       utc_time:  Option<chrono::DateTime<chrono::Utc>>,
       "UTC time of the GGA or GNS fix associated with this sentence."
   },
   {
       latitude_error:Option<f64>,
       "Expected 1-sigma error in latitude (meters)"
   },
   {
       longitude_error:Option<f64>,
       "Expected 1-sigma error in longitude (meters)"
   },
   {
       altitude_error:Option<f64>,
       "Expected 1-sigma error in altitude (meters)"
   },
   {
       likely_failed_satellite_id:Option<u16>,
       "ID of most likely failed satellite (1 to 138)"
   },
   {
       missed_detection_probability:Option<f64>,
       "Probability of missed detection for most likely failed satellite"
   },
   {
       bias_estimate:Option<f64>,
       "Estimate of bias in meters on most likely failed satellite"
   },
   {
       std_dev:Option<f64>,
       "Standard deviation of bias estimate"
   }
);

impl Gbs {
    pub fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        let utc_time = ctx
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .take(&NMEA_UTC);
        let latitude_error = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let longitude_error = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let altitude_error = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let likely_failed_satellite_id =
            ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let missed_detection_probability =
            ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let bias_estimate = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let std_dev = ctx.skip(&CHAR_COMMA).take(&UNTIL_STAR).parse_opt();

        Ok(Gbs {
            talker,
            utc_time,
            latitude_error,
            longitude_error,
            altitude_error,
            likely_failed_satellite_id,
            missed_detection_probability,
            bias_estimate,
            std_dev,
        })
    }
}
impl Debug for Gbs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("GBS");
        ds.field("talker", &self.talker);

        if let Some(ref utc_time) = self.utc_time {
            ds.field("utc_time", utc_time);
        }
        if let Some(latitude_error) = self.latitude_error {
            ds.field("latitude_error", &latitude_error);
        }
        if let Some(longitude_error) = self.longitude_error {
            ds.field("longitude_error", &longitude_error);
        }
        if let Some(altitude_error) = self.altitude_error {
            ds.field("altitude_error", &altitude_error);
        }
        if let Some(likely_failed_satellite_id) = self.likely_failed_satellite_id {
            ds.field("likely_failed_satellite_id", &likely_failed_satellite_id);
        }
        if let Some(missed_detection_probability) = self.missed_detection_probability {
            ds.field(
                "missed_detection_probability",
                &missed_detection_probability,
            );
        }
        if let Some(bias_estimate) = self.bias_estimate {
            ds.field("bias_estimate", &bias_estimate);
        }
        if let Some(std_dev) = self.std_dev {
            ds.field("std_dev", &std_dev);
        }

        ds.finish()
    }
}
#[cfg(test)]
mod tests {

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;

    #[test]
    fn test_gbs() {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPGBS,125027,23.43,M,13.91,M,34.01,M*07";
        let mut ctx = StrParserContext::new();
        let gbs = Gbs::new(ctx.init(s.to_string()), Talker::GP).unwrap();
        println!("{gbs:?}");
        assert_eq!(gbs.talker, Talker::GP);
        assert!(gbs.utc_time.unwrap().to_string().contains("12:50:27"));
        assert_eq!(gbs.latitude_error.unwrap(), 23.43);
        assert_eq!(gbs.altitude_error.unwrap(), 13.91);
        assert_eq!(gbs.missed_detection_probability.unwrap(), 34.01);
    }
}
