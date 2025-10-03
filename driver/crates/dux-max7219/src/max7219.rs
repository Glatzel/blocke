use core::marker::PhantomData;

use embedded_hal::delay::DelayNs;
use embedded_hal::spi::{Error, SpiDevice};

use crate::error::Max7219Error;
use crate::register::Register;
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Max7219<SPI, D> {
    count: usize,
    spi: SPI,
    _delay: PhantomData<D>,
}
impl<SPI, D> Max7219<SPI, D>
where
    SPI: SpiDevice,
    D: DelayNs,
{
    pub fn write(
        &mut self,
        index: usize,
        register: Register,
        data: u8,
    ) -> Result<(), Max7219Error> {
        let mut buffer = Vec::with_capacity(self.count * 2);

        for i in 0..self.count {
            if i == index {
                buffer.push(register.code());
                buffer.push(data);
            } else {
                buffer.push(0x00);
                buffer.push(0x00);
            }
        }

        self.spi
            .write(&buffer)
            .map_err(|e| Max7219Error::SpiWriteError(e.kind()))
    }
}
