use embedded_hal::delay::DelayNs;
use embedded_hal::spi::SpiDevice;

use crate::Max7219;
use crate::error::Max7219Error;

impl<SPI, D> Max7219<SPI, D>
where
    SPI: SpiDevice,
    D: DelayNs,
{
    pub fn show_matrix_line(&mut self, line_index: u8, spot: u8) -> Result<(), Max7219Error> {
        self.write(line_index.try_into()?, spot)
    }
}
