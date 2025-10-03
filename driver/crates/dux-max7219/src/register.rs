use crate::error::Max7219Error;

pub enum Register {
    NoOp,
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    DecodeMode,
    Intensity,
    ScanLimit,
    Shutdown,
    DisplayTest,
}
impl TryFrom<u8> for Register {
    type Error = Max7219Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let v = match value {
            0 => Self::D0,
            1 => Self::D1,
            2 => Self::D2,
            3 => Self::D3,
            4 => Self::D4,
            5 => Self::D5,
            6 => Self::D6,
            7 => Self::D7,
            i => return Err(Max7219Error::InvalidIndex(i)),
        };
        Ok(v)
    }
}
impl Register {
    pub fn code(&self) -> u8 {
        match self {
            Register::NoOp => 0x00,
            Register::D0 => 0x01,
            Register::D1 => 0x02,
            Register::D2 => 0x03,
            Register::D3 => 0x04,
            Register::D4 => 0x05,
            Register::D5 => 0x06,
            Register::D6 => 0x07,
            Register::D7 => 0x08,
            Register::DecodeMode => 0x09,
            Register::Intensity => 0x0A,
            Register::ScanLimit => 0x0B,
            Register::Shutdown => 0x0C,
            Register::DisplayTest => 0x0F,
        }
    }
}
