use embedded_hal::spi::ErrorKind;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum Max7219Error {
    #[error("Failed I2C write: {0:?}")]
    SpiWriteError(ErrorKind),
    #[error(" Invalid segment char(: {0}")]
    InvalidSegmentChar(char),
    #[error(" Invalid index(: {0}")]
    InvalidIndex(u8),
}
