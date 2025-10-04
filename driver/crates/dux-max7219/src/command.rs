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
    Shutdown,
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
            Command::Shutdown => 0x0C,
            Command::DisplayTest => 0x0F,
        }
    }
}
