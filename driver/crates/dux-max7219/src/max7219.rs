#[cfg(not(feature = "async"))]
use embedded_hal::spi::{Error, SpiDevice};
#[cfg(feature = "async")]
use embedded_hal_async::spi::{Error, SpiDevice};
use paste::paste;

use crate::SegmentChar;
use crate::error::Max7219Error;
use crate::primitives::{Command, DecodeMode, Intensity, ScanLimit};
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Hash)]
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
#[cfg(not(feature = "async"))]
macro_rules! impl_setter {
    ($name:ident, $cmd:ident, $ty:ty) => {
        paste! {
            pub const fn $name(&mut self, index: usize, value: $ty) -> Result<&mut Self, Max7219Error> {
               self.command(index, Command::$cmd, value as u8)
            }
            pub fn [<$name _checked>](
                &mut self,
                index: usize,
                value: $ty,
            ) -> Result<&mut Self, Max7219Error> {
                self.command_checked(index, Command::$cmd, value as u8)
            }
        }
    };
}
#[cfg(feature = "async")]
macro_rules! impl_setter {
    ($name:ident, $cmd:ident, $ty:ty) => {
        paste! {
            pub const fn $name(&mut self, index: usize, value: $ty) -> Result<&mut Self, Max7219Error> {
               self.command(index, Command::$cmd, value as u8)
            }
            pub async fn [<$name _checked>](
                &mut self,
                index: usize,
                value: $ty,
            ) -> Result<&mut Self, Max7219Error> {
                self.command_checked(index, Command::$cmd, value as u8).await
            }
        }
    };
}

impl<SPI, const N: usize, const BUF: usize> Max7219<SPI, N, BUF>
where
    SPI: SpiDevice,
{
    pub const fn count(&self) -> usize { N }

    pub const fn command(
        &mut self,
        index: usize,
        command: Command,
        data: u8,
    ) -> Result<&mut Self, Max7219Error> {
        self.buffer[2 * index] = data;
        self.buffer[2 * index + 1] = command as u8;

        self.updated[index] = true;
        Ok(self)
    }

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

#[cfg(not(feature = "async"))]
impl<SPI, const N: usize, const BUF: usize> Max7219<SPI, N, BUF>
where
    SPI: SpiDevice,
{
    pub fn write(&mut self) -> Result<&mut Self, Max7219Error> {
        self.device
            .write(&self.buffer)
            .map_err(|e| Max7219Error::SpiWriteError(e.kind()))?;
        self.updated.fill(false);
        Ok(self)
    }
    pub fn command_checked(
        &mut self,
        index: usize,
        command: Command,
        data: u8,
    ) -> Result<&mut Self, Max7219Error> {
        if index >= N {
            return Err(Max7219Error::IndexOutOfBounds { index, bound: N });
        }
        if self.updated[index] {
            self.write()?;
        }
        self.command(index, command, data)
    }
    impl_setter!(set_power, Power, bool);
    impl_setter!(set_decode_mode, DecodeMode, DecodeMode);
    impl_setter!(set_intensity, Intensity, Intensity);
    impl_setter!(set_scan_limit, ScanLimit, ScanLimit);
    impl_setter!(set_display_test, DisplayTest, bool);
    pub fn set_digit_checked(
        &mut self,
        device_index: usize,
        digit_index: u8,
        data: u8,
    ) -> Result<&mut Self, Max7219Error> {
        self.command_checked(device_index, digit_index.try_into()?, data)
    }
    pub fn set_segment_char_checked(
        &mut self,
        device_index: usize,
        segment_index: u8,
        character: SegmentChar,
        with_dot: bool,
    ) -> Result<&mut Self, Max7219Error> {
        self.set_digit_checked(
            device_index,
            segment_index,
            character as u8 | ((with_dot as u8) << 7),
        )
    }
}
#[cfg(feature = "async")]
impl<SPI, const N: usize, const BUF: usize> Max7219<SPI, N, BUF>
where
    SPI: SpiDevice,
{
    pub async fn write(&mut self) -> Result<&mut Self, Max7219Error> {
        self.device
            .write(&self.buffer)
            .await
            .map_err(|e| Max7219Error::SpiWriteError(e.kind()))?;
        self.updated.fill(false);
        Ok(self)
    }
    pub async fn command_checked(
        &mut self,
        index: usize,
        command: Command,
        data: u8,
    ) -> Result<&mut Self, Max7219Error> {
        if index >= N {
            return Err(Max7219Error::IndexOutOfBounds { index, bound: N });
        }
        if self.updated[index] {
            self.write().await?;
        }
        self.command(index, command, data)
    }
    impl_setter!(set_power, Power, bool);
    impl_setter!(set_decode_mode, DecodeMode, DecodeMode);
    impl_setter!(set_intensity, Intensity, Intensity);
    impl_setter!(set_scan_limit, ScanLimit, ScanLimit);
    impl_setter!(set_display_test, DisplayTest, bool);
    pub async fn set_digit_checked(
        &mut self,
        device_index: usize,
        digit_index: u8,
        data: u8,
    ) -> Result<&mut Self, Max7219Error> {
        self.command_checked(device_index, digit_index.try_into()?, data)
            .await
    }
    pub async fn set_segment_char_checked(
        &mut self,
        device_index: usize,
        segment_index: u8,
        character: SegmentChar,
        with_dot: bool,
    ) -> Result<&mut Self, Max7219Error> {
        self.set_digit_checked(
            device_index,
            segment_index,
            character as u8 | ((with_dot as u8) << 7),
        )
        .await
    }
}
