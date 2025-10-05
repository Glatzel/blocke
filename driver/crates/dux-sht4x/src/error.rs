use embedded_hal::i2c::{Error, ErrorKind};
use sensirion_i2c::i2c;
use thiserror::Error;

/// Error conditions from accessing SHT4x sensors.
#[derive(Debug, Error)]
pub enum Sht4xError {
    #[error("Failed I2C read: {0:?}")]
    I2cRead(ErrorKind),
    #[error("Failed I2C write: {0:?}")]
    I2cWrite(ErrorKind),
    #[error("Failed CRC verification of sensor data.")]
    Crc,
}

impl<E> From<i2c::Error<E>> for Sht4xError
where
    E: embedded_hal::i2c::ErrorType,
{
    fn from(err: i2c::Error<E>) -> Self {
        match err {
            i2c::Error::Crc => Sht4xError::Crc,
            i2c::Error::I2cRead(e) => Sht4xError::I2cRead(e.kind()),
            i2c::Error::I2cWrite(e) => Sht4xError::I2cWrite(e.kind()),
        }
    }
}
