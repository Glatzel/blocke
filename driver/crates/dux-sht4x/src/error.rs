use sensirion_i2c::i2c;
use thiserror::Error;

/// Error conditions from accessing SHT4x sensors.

#[derive(Debug, Error)]
pub enum Sht4xError<E> {
    /// Failed I2C Read.
    I2cRead(E),
    /// Failed I2C Write.
    I2cWrite(E),
    /// Failed CRC verification of sensor data.
    Crc,
}

impl<E> From<i2c::Error<E>> for Sht4xError<E::Error>
where
    E: embedded_hal::i2c::ErrorType,
{
    fn from(err: i2c::Error<E>) -> Self {
        match err {
            i2c::Error::Crc => Sht4xError::Crc,
            i2c::Error::I2cRead(e) => Sht4xError::I2cRead(e),
            i2c::Error::I2cWrite(e) => Sht4xError::I2cWrite(e),
        }
    }
}
