mod nmea_coord;
mod nmea_date;
mod nmea_utc;
mod nmea_validate;

use std::sync::LazyLock;

use rax_parser::str_parser::rules::{Char, Until};

pub use crate::rules::nmea_coord::NmeaCoord;
pub use crate::rules::nmea_date::NmeaDate;
pub use crate::rules::nmea_utc::NmeaUtc;
pub use crate::rules::nmea_validate::NmeaValidate;

pub const CHAR_COMMA: LazyLock<Char> = LazyLock::new(|| Char(&','));
pub const CHAR_NEW_LINE: LazyLock<Char> = LazyLock::new(|| Char(&'\n'));
pub const CHAR_M: LazyLock<Char> = LazyLock::new(|| Char(&'M'));

pub const UNTIL_COMMA: LazyLock<Until> = LazyLock::new(|| Until(","));
pub const UNTIL_STAR: LazyLock<Until> = LazyLock::new(|| Until("*"));
pub const UNTIL_NEW_LINE: LazyLock<Until> = LazyLock::new(|| Until("\n"));

pub const NMEA_COORD: LazyLock<NmeaCoord> = LazyLock::new(NmeaCoord);
pub const NMEA_DATE: LazyLock<NmeaDate> = LazyLock::new(NmeaDate);
pub const NMEA_UTC: LazyLock<NmeaUtc> = LazyLock::new(NmeaUtc);
pub const NMEA_VALIDATE: LazyLock<NmeaValidate> = LazyLock::new(NmeaValidate);
