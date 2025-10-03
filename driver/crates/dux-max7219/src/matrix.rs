use embedded_hal::spi::SpiDevice;

use crate::Max7219;
use crate::error::Max7219Error;

impl<SPI> Max7219<SPI>
where
    SPI: SpiDevice,
{
    pub fn show_matrix_line(
        &mut self,
        line_index: u8,
        pattern: u8,
    ) -> Result<(), Max7219Error> {
        self.write(line_index.try_into()?, pattern)
    }
}
