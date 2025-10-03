use core::marker::PhantomData;

use embedded_hal::delay::DelayNs;
use embedded_hal::spi::{Error, SpiDevice};

use crate::error::Max7219Error;
use crate::register::Register;
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Max7219<SPI, D> {
    spi: SPI,
    _delay: PhantomData<D>,
}
impl<SPI, D> Max7219<SPI, D>
where
    SPI: SpiDevice,
    D: DelayNs,
{
    pub fn write(&mut self, register: Register, data: u8) -> Result<(), Max7219Error> {
        self.spi
            .write(&[register.code(), data])
            .map_err(|e| Max7219Error::SpiWriteError(e.kind()))?;
        Ok(())
    }
}
