use crate::error::Max7219Error;

#[repr(u8)]
pub enum SegmentChar {
    Zero = 0b0000,
    One = 0b0001,
    Two = 0b0010,
    Three = 0b0011,
    Four = 0b0100,
    Five = 0b0101,
    Six = 0b0110,
    Sieven = 0b0111,
    Eight = 0b1000,
    Nine = 0b1001,
    Dash = 0b1010,
    E = 0b1011,
    H = 0b1100,
    L = 0b1101,
    P = 0b1110,
    Blank = 0b1111,
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
