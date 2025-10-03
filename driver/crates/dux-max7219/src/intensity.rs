use embedded_hal::spi::SpiDevice;

use crate::Max7219;
use crate::error::Max7219Error;

pub enum Intensity {
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
    L8,
    L9,
    L10,
    L11,
    L12,
    L13,
    L14,
    L15,
    L16,
}
impl Intensity {
    pub(crate) fn data(&self) -> u8 {
        match self {
            Intensity::L1 => 0,
            Intensity::L2 => 1,
            Intensity::L3 => 2,
            Intensity::L4 => 3,
            Intensity::L5 => 4,
            Intensity::L6 => 5,
            Intensity::L7 => 6,
            Intensity::L8 => 7,
            Intensity::L9 => 8,
            Intensity::L10 => 9,
            Intensity::L11 => 10,
            Intensity::L12 => 11,
            Intensity::L13 => 12,
            Intensity::L14 => 13,
            Intensity::L15 => 14,
            Intensity::L16 => 15,
        }
    }
}
impl<SPI, const N: usize> Max7219<SPI, N>
where
    SPI: SpiDevice,
{
    pub fn set_intensity(
        &mut self,
        index: usize,
        intensity: Intensity,
    ) -> Result<(), Max7219Error> {
        self.write_manual(
            index,
            crate::register::Register::Intensity,
            intensity.data(),
        )
    }
}
