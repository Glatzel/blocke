use embedded_hal::spi::{Error, SpiDevice};

use crate::error::Max7219Error;
use crate::register::Register;
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Max7219<SPI> {
    spi: SPI,
}
impl<SPI> Max7219<SPI>
where
    SPI: SpiDevice,
{
    pub fn new(spi: SPI) -> Self { Self { spi } }
    pub fn write(&mut self, register: Register, data: u8) -> Result<(), Max7219Error> {
        self.spi
            .write(&[data, register.code()])
            .map_err(|e| Max7219Error::SpiWriteError(e.kind()))
    }
}
