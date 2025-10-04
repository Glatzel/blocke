use crate::error::Max7219Error;

pub enum Command {
    NoOp,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    DecodeMode,
    Intensity,
    ScanLimit,
    Power,
    DisplayTest,
}
impl TryFrom<u8> for Command {
    type Error = Max7219Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let v = match value {
            0 => Self::Digit0,
            1 => Self::Digit1,
            2 => Self::Digit2,
            3 => Self::Digit3,
            4 => Self::Digit4,
            5 => Self::Digit5,
            6 => Self::Digit6,
            7 => Self::Digit7,
            i => return Err(Max7219Error::InvalidIndex(i)),
        };
        Ok(v)
    }
}
impl Command {
    pub fn code(&self) -> u8 {
        match self {
            Command::NoOp => 0x00,
            Command::Digit0 => 0x01,
            Command::Digit1 => 0x02,
            Command::Digit2 => 0x03,
            Command::Digit3 => 0x04,
            Command::Digit4 => 0x05,
            Command::Digit5 => 0x06,
            Command::Digit6 => 0x07,
            Command::Digit7 => 0x08,
            Command::DecodeMode => 0x09,
            Command::Intensity => 0x0A,
            Command::ScanLimit => 0x0B,
            Command::Power => 0x0C,
            Command::DisplayTest => 0x0F,
        }
    }
}

pub enum DecodeMode {
    NoDecode,
    CodeBDigit0,
    CodeBDigits3_0,
    CodeBDigits7_0,
}
impl DecodeMode {
    pub fn data(&self) -> u8 {
        match self {
            DecodeMode::NoDecode => 0x00,
            DecodeMode::CodeBDigit0 => 0x01,
            DecodeMode::CodeBDigits3_0 => 0x0F,
            DecodeMode::CodeBDigits7_0 => 0xFF,
        }
    }
}
pub enum Intensity {
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
    L8,
    L9,
    L10,
    L11,
    L12,
    L13,
    L14,
    L15,
    L16,
}
impl Intensity {
    pub fn data(&self) -> u8 {
        match self {
            Intensity::L1 => 0x00,
            Intensity::L2 => 0x01,
            Intensity::L3 => 0x02,
            Intensity::L4 => 0x03,
            Intensity::L5 => 0x04,
            Intensity::L6 => 0x05,
            Intensity::L7 => 0x06,
            Intensity::L8 => 0x07,
            Intensity::L9 => 0x08,
            Intensity::L10 => 0x09,
            Intensity::L11 => 0x0A,
            Intensity::L12 => 0x0B,
            Intensity::L13 => 0x0C,
            Intensity::L14 => 0x0D,
            Intensity::L15 => 0x0E,
            Intensity::L16 => 0x0F,
        }
    }
}
pub enum ScanLimit {
    DisplayDigit0,
    DisplayDigit0_1,
    DisplayDigit0_2,
    DisplayDigit0_3,
    DisplayDigit0_4,
    DisplayDigit0_5,
    DisplayDigit0_6,
    DisplayDigit0_7,
}
impl ScanLimit {
    pub fn data(&self) -> u8 {
        match self {
            ScanLimit::DisplayDigit0 => 0x00,
            ScanLimit::DisplayDigit0_1 => 0x01,
            ScanLimit::DisplayDigit0_2 => 0x02,
            ScanLimit::DisplayDigit0_3 => 0x03,
            ScanLimit::DisplayDigit0_4 => 0x04,
            ScanLimit::DisplayDigit0_5 => 0x05,
            ScanLimit::DisplayDigit0_6 => 0x06,
            ScanLimit::DisplayDigit0_7 => 0x07,
        }
    }
}
