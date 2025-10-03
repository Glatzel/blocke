use crate::error::Max7219Error;

pub enum SegmentChar {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Sieven,
    Eight,
    Nine,
    Dash,
    E,
    H,
    L,
    P,
    Blank,
}
impl TryFrom<char> for SegmentChar {
    type Error = Max7219Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let result = match value {
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Sieven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            '-' => Self::Dash,
            'E' => Self::E,
            'H' => Self::H,
            'L' => Self::L,
            'P' => Self::P,
            ' ' => Self::Blank,
            _ => return Err(Max7219Error::InvalidSegmentChar(value)),
        };
        Ok(result)
    }
}
impl SegmentChar {
    fn data(&self, with_dot: bool) -> u8 {
        let base = match self {
            SegmentChar::Zero => 0b0000,
            SegmentChar::One => 0b0001,
            SegmentChar::Two => 0b0010,
            SegmentChar::Three => 0b0011,
            SegmentChar::Four => 0b0100,
            SegmentChar::Five => 0b0101,
            SegmentChar::Six => 0b0110,
            SegmentChar::Sieven => 0b0111,
            SegmentChar::Eight => 0b1000,
            SegmentChar::Nine => 0b1001,
            SegmentChar::Dash => 0b1010,
            SegmentChar::E => 0b0000,
            SegmentChar::H => 0b0000,
            SegmentChar::L => 0b0000,
            SegmentChar::P => 0b1110,
            SegmentChar::Blank => 0b1111,
        };
        base | ((with_dot as u8) << 7)
    }
}
