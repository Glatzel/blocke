mod nmea_coord;
mod nmea_date;
mod nmea_degree;
mod nmea_utc;
mod nmea_validate;
use rax_parser::str_parser::rules::{Char, Until};

use crate::rules::nmea_coord::NmeaCoord;
use crate::rules::nmea_date::NmeaDate;
use crate::rules::nmea_utc::NmeaUtc;
use crate::rules::nmea_validate::NmeaValidate;
use crate::rules::nmea_degree::NmeaDegree;

pub const CHAR_COMMA: Char<','> = Char();
pub const CHAR_NEW_LINE: Char<'\n'> = Char();
pub const CHAR_M: Char<'M'> = Char();
pub const CHAR_K: Char<'K'> = Char();
pub const CHAR_T: Char<'T'> = Char();
pub const CHAR_N: Char<'N'> = Char();

pub const UNTIL_COMMA: Until = Until(",");
pub const UNTIL_STAR: Until = Until("*");
pub const UNTIL_NEW_LINE: Until = Until("\n");

pub const NMEA_COORD: NmeaCoord = NmeaCoord();
pub const NMEA_DATE: NmeaDate = NmeaDate();
pub const NMEA_UTC: NmeaUtc = NmeaUtc();
pub const NMEA_VALIDATE: NmeaValidate = NmeaValidate();
pub const NMEA_DEGREE: NmeaDegree = NmeaDegree();