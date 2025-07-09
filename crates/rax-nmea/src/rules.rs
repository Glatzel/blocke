mod nmea_coord;
mod nmea_date;
mod nmea_degree;
mod nmea_utc;
mod nmea_validate;
use rax::str_parser::filters::CharSetFilter;
use rax::str_parser::rules::{Char, CharCount, Until, UntilMode, UntilOneInCharSet};

use crate::rules::nmea_coord::NmeaCoord;
use crate::rules::nmea_date::NmeaDate;
use crate::rules::nmea_degree::NmeaDegree;
use crate::rules::nmea_utc::NmeaUtc;
use crate::rules::nmea_validate::NmeaValidate;

pub const CHAR_COMMA: Char<','> = Char;
pub const CHAR_NEW_LINE: Char<'\n'> = Char;
pub const CHAR_M: Char<'M'> = Char;
pub const CHAR_T: Char<'T'> = Char;
pub const CHAR_N: Char<'N'> = Char;

pub const UNTIL_COMMA_DISCARD: Until = Until {
    delimiter: ",",
    mode: UntilMode::Discard,
};
pub const UNTIL_STAR_DISCARD: Until = Until {
    delimiter: "*",
    mode: UntilMode::Discard,
};
pub const UNTIL_NEW_LINE_DISCARD: Until = Until {
    delimiter: "\n",
    mode: UntilMode::Discard,
};

pub const NMEA_COORD: NmeaCoord = NmeaCoord();
pub const NMEA_DATE: NmeaDate = NmeaDate();
pub const NMEA_UTC: NmeaUtc = NmeaUtc();
pub const NMEA_VALIDATE: NmeaValidate = NmeaValidate();
pub const NMEA_DEGREE: NmeaDegree = NmeaDegree();

pub const UNTIL_COMMA_OR_STAR_DISCARD: UntilOneInCharSet<2> = UntilOneInCharSet {
    filter: &CharSetFilter::new([',', '*']),
    mode: UntilMode::Discard,
};

pub const THREE_CHAR: CharCount<3> = CharCount;
