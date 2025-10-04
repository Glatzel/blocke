use embedded_hal::spi::{Error, SpiDevice};

use crate::SegmentChar;
use crate::error::Max7219Error;
use crate::primitives::{Command, DecodeMode, Intensity, ScanLimit};

#[derive(Debug)]
pub struct Max7219<SPI: SpiDevice, const N: usize, const BUF: usize> {
    device: SPI,
    buffer: [u8; BUF],
    updated: [bool; N],
}
#[macro_export]
macro_rules! max7219_new {
    ($device:expr,$count:expr) => {
        const N: usize = $count;
        const BUF: usize = $count * 2;
        Max7219 {
            $device,
            buffer: [0; N],
            updated: [false; BUF],
        }
    };
}
macro_rules! impl_setter {
    ($name:ident, $cmd:ident, $ty:ty) => {
        pub fn $name(&mut self, index: usize, value: $ty) -> Result<&mut Self, Max7219Error> {
            self.command(index, Command::$cmd, value as u8)
        }
    };
}

impl<SPI, const N: usize, const BUF: usize> Max7219<SPI, N, BUF>
where
    SPI: SpiDevice,
{
    pub const fn count(&self) -> usize { N }
    pub fn write(&mut self) -> Result<&mut Self, Max7219Error> {
        self.device
            .write(&self.buffer)
            .map_err(|e| Max7219Error::SpiWriteError(e.kind()))?;
        self.updated.fill(false);
        Ok(self)
    }
    pub fn command(
        &mut self,
        index: usize,
        command: Command,
        data: u8,
    ) -> Result<&mut Self, Max7219Error> {
        if index >= N {
            return Err(Max7219Error::IndexOutOfBounds { index, bound: N });
        }
        if self.updated[index] == true {
            self.write()?;
        }

        self.buffer[2 * index] = data;
        self.buffer[2 * index + 1] = command as u8;

        self.updated[index] = true;
        Ok(self)
    }
    impl_setter!(set_power, Power, bool);
    impl_setter!(set_decode_mode, DecodeMode, DecodeMode);
    impl_setter!(set_intensity, Intensity, Intensity);
    impl_setter!(set_scan_limit, ScanLimit, ScanLimit);
    impl_setter!(set_display_test, DisplayTest, bool);
    pub fn set_digit(
        &mut self,
        device_index: usize,
        digit_index: u8,
        data: u8,
    ) -> Result<&mut Self, Max7219Error> {
        self.command(device_index, digit_index.try_into()?, data)
    }
    pub fn set_segment_char(
        &mut self,
        device_index: usize,
        segment_index: u8,
        character: SegmentChar,
        with_dot: bool,
    ) -> Result<&mut Self, Max7219Error> {
        self.set_digit(
            device_index,
            segment_index,
            character as u8 | ((with_dot as u8) << 7),
        )
    }
}
impl<SPI, const N: usize, const BUF: usize> Drop for Max7219<SPI, N, BUF>
where
    SPI: SpiDevice,
{
    fn drop(&mut self) { self.write().unwrap(); }
}
