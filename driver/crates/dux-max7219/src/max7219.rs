use embedded_hal::spi::{Error, SpiDevice};

use crate::error::Max7219Error;
use crate::register::Register;
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Max7219<SPI, const N: usize> {
    spi: SPI,
}
impl<SPI, const N: usize> Max7219<SPI, N>
where
    SPI: SpiDevice,
{
    pub fn new(spi: SPI) -> Self { Self { spi } }
    pub fn write(&mut self, buffer: &[u8]) -> Result<(), Max7219Error> {
        self.spi
            .write(buffer)
            .map_err(|e| Max7219Error::SpiWriteError(e.kind()))
    }
    pub fn write_all(&mut self, register: Register, data: u8) -> Result<(), Max7219Error> {
        let mut buffer = Vec::with_capacity(N * 2);

        for _ in 0..N {
            buffer.push(register.code());
            buffer.push(data);
        }

        self.spi
            .write(&buffer)
            .map_err(|e| Max7219Error::SpiWriteError(e.kind()))
    }
    pub fn write_manual(
        &mut self,
        index: usize,
        register: Register,
        data: u8,
    ) -> Result<(), Max7219Error> {
        let mut buffer = Vec::with_capacity(N * 2);

        for i in 0..N {
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
