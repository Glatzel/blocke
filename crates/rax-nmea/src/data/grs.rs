use std::fmt;
use std::str::FromStr;

use rax_parser::str_parser::{ParseOptExt, StrParserContext};
use serde::{Deserialize, Serialize};

use crate::data::{INmeaData, SystemId, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum GrsResidualMode {
    UsedInGga,
    CalculatedAfterGga,
}
impl FromStr for GrsResidualMode {
    type Err = miette::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::UsedInGga),
            "1" => Ok(Self::CalculatedAfterGga),
            other => miette::bail!("Unknown GrsResidualMode: {}", other),
        }
    }
}
readonly_struct!(
    Grs ,
    "Gsa",
    {talker: Talker},

    {utc_time: Option<chrono::DateTime<chrono::Utc>>},
    {grs_residual_mode : Option<GrsResidualMode>},
    {satellite_residuals:Vec<f64>},
    {system_id: Option<SystemId>},
    {signal_id: Option<u16>}
);
impl INmeaData for Grs {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;

        let utc_time = ctx
            .skip_strict(&UNTIL_COMMA)?
            .skip_strict(&CHAR_COMMA)?
            .take(&NMEA_UTC);

        let grs_residual_mode = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        clerk::debug!(
            "Grs::new: utc_time={:?}, grs_residual_mode={:?}",
            utc_time,
            grs_residual_mode
        );

        let mut satellite_residuals = Vec::with_capacity(12);
        for _ in 0..12 {
            match ctx
                .skip_strict(&CHAR_COMMA)?
                .take(&UNTIL_COMMA)
                .parse_opt::<f64>()
            {
                Some(residual) => satellite_residuals.push(residual),
                None => continue,
            }
        }
        clerk::debug!("Grs::new: satellite_residuals={:?}", satellite_residuals);

        let system_id = ctx.skip_strict(&CHAR_COMMA)?.take(&UNTIL_COMMA).parse_opt();
        let signal_id = ctx.skip(&CHAR_COMMA).take(&UNTIL_STAR).parse_opt();
        Ok(Grs {
            talker,
            utc_time,
            grs_residual_mode,
            satellite_residuals,
            system_id,
            signal_id,
        })
    }
}

impl fmt::Debug for Grs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("GSA");
        ds.field("talker", &self.talker);

        if let Some(utc_time) = self.utc_time {
            ds.field("utc_time", &utc_time);
        }

        if let Some(grs_residual_mode) = self.grs_residual_mode {
            ds.field("grs_residual_mode", &grs_residual_mode);
        }

        ds.field("satellite_residuals", &self.satellite_residuals);

        if let Some(system_id) = self.system_id {
            ds.field("system_id", &system_id);
        }

        if let Some(signal_id) = self.signal_id {
            ds.field("signal_id", &signal_id);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {

    use clerk::init_log_with_level;
    use clerk::tracing::level_filters::LevelFilter;
    use float_cmp::assert_approx_eq;

    use super::*;
    #[test]
    fn test_grs() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let input = "$GPGRS,220320.0,0,-0.8,-0.2,-0.1,-0.2,0.8,0.6,,,,,,,*55";
        let mut ctx = StrParserContext::new();
        let grs = Grs::new(ctx.init(input.to_string()), Talker::GP)?;
        println!("{:?}", grs);

        assert_eq!(grs.talker, Talker::GP);
        assert!(grs.utc_time.unwrap().to_string().contains("22:03:20"));
        assert_eq!(grs.grs_residual_mode.unwrap(), GrsResidualMode::UsedInGga);
        assert_eq!(grs.satellite_residuals.len(), 6);
        assert_approx_eq!(f64, grs.satellite_residuals[0], -0.8);
        assert_approx_eq!(f64, grs.satellite_residuals[1], -0.2);
        assert_approx_eq!(f64, grs.satellite_residuals[2], -0.1);
        assert_approx_eq!(f64, grs.satellite_residuals[3], -0.2);
        assert_approx_eq!(f64, grs.satellite_residuals[4], 0.8);
        assert_approx_eq!(f64, grs.satellite_residuals[5], 0.6);
        assert!(grs.system_id.is_none());
        assert!(grs.signal_id.is_none());

        Ok(())
    }
}
