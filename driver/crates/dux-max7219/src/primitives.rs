use crate::error::Max7219Error;

#[repr(u8)]
pub enum Command {
    NoOp = 0x00,
    Digit0 = 0x01,
    Digit1 = 0x02,
    Digit2 = 0x03,
    Digit3 = 0x04,
    Digit4 = 0x05,
    Digit5 = 0x06,
    Digit6 = 0x07,
    Digit7 = 0x08,
    DecodeMode = 0x09,
    Intensity = 0x0A,
    ScanLimit = 0x0B,
    Power = 0x0C,
    DisplayTest = 0x0F,
}

impl TryFrom<u8> for Command {
    type Error = Max7219Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Digit0),
            1 => Ok(Self::Digit1),
            2 => Ok(Self::Digit2),
            3 => Ok(Self::Digit3),
            4 => Ok(Self::Digit4),
            5 => Ok(Self::Digit5),
            6 => Ok(Self::Digit6),
            7 => Ok(Self::Digit7),
            i => Err(Max7219Error::InvalidIndex(i)),
        }
    }
}

#[repr(u8)]
pub enum DecodeMode {
    NoDecode = 0x00,
    CodeBDigit0 = 0x01,
    CodeBDigits3_0 = 0x0F,
    CodeBDigits7_0 = 0xFF,
}

#[repr(u8)]
pub enum Intensity {
    L1 = 0x00,
    L2 = 0x01,
    L3 = 0x02,
    L4 = 0x03,
    L5 = 0x04,
    L6 = 0x05,
    L7 = 0x06,
    L8 = 0x07,
    L9 = 0x08,
    L10 = 0x09,
    L11 = 0x0A,
    L12 = 0x0B,
    L13 = 0x0C,
    L14 = 0x0D,
    L15 = 0x0E,
    L16 = 0x0F,
}

#[repr(u8)]
pub enum ScanLimit {
    DisplayDigit0 = 0x00,
    DisplayDigit0_1 = 0x01,
    DisplayDigit0_2 = 0x02,
    DisplayDigit0_3 = 0x03,
    DisplayDigit0_4 = 0x04,
    DisplayDigit0_5 = 0x05,
    DisplayDigit0_6 = 0x06,
    DisplayDigit0_7 = 0x07,
}
