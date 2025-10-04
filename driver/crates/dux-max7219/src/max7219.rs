use embedded_hal::spi::{Error, SpiDevice};

use crate::error::Max7219Error;
use crate::primitives::{Command, Intensity, PowerMode};
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Max7219<SPI> {
    spi: SPI,
}
impl<SPI> Max7219<SPI>
where
    SPI: SpiDevice,
{
    pub fn new(spi: SPI) -> Self { Self { spi } }
    pub fn write(&mut self, command: Command, data: u8) -> Result<(), Max7219Error> {
        self.spi
            .write(&[data, command])
            .map_err(|e| Max7219Error::SpiWriteError(e.kind()))
    }
    pub fn set_intensity(&mut self, intensity: Intensity) -> Result<(), Max7219Error> {
        self.write(crate::primitives::Command::Intensity, intensity.data())
    }
    pub fn power(&mut self, mode: PowerMode) -> Result<(), Max7219Error> {
        self.write(crate::primitives::Command::Intensity, mode.data())
    }
}
