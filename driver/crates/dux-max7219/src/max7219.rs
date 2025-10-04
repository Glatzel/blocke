use embedded_hal::spi::{Error, SpiDevice};

use crate::error::Max7219Error;
use crate::primitives::{Command, DecodeMode, Intensity, ScanLimit};
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Max7219<SPI> {
    device: SPI,
}
impl<SPI> Max7219<SPI>
where
    SPI: SpiDevice,
{
    pub fn new(spi: SPI) -> Self { Self { device: spi } }
    pub fn write_raw(&mut self, command: Command, data: u8) -> Result<(), Max7219Error> {
        self.device
            .write(&[data, command.code()])
            .map_err(|e| Max7219Error::SpiWriteError(e.kind()))
    }
    pub fn set_power(&mut self, enable: bool) -> Result<(), Max7219Error> {
        self.write_raw(crate::primitives::Command::Power, enable as u8)
    }
    pub fn set_decode_mode(&mut self, mode: DecodeMode) -> Result<(), Max7219Error> {
        self.write_raw(crate::primitives::Command::DecodeMode, mode.data())
    }
    pub fn set_intensity(&mut self, intensity: Intensity) -> Result<(), Max7219Error> {
        self.write_raw(crate::primitives::Command::Intensity, intensity.data())
    }
    pub fn set_scan_limit(&mut self, limit: ScanLimit) -> Result<(), Max7219Error> {
        self.write_raw(crate::primitives::Command::ScanLimit, limit.data())
    }
    pub fn set_display_test(&mut self, enable: bool) -> Result<(), Max7219Error> {
        self.write_raw(crate::primitives::Command::DisplayTest, enable as u8)
    }
}
