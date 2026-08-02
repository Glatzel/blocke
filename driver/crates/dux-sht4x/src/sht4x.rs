use core::marker::PhantomData;
#[cfg(feature = "async")]
mod impl_async;
#[cfg(not(feature = "async"))]
mod impl_sync;
#[cfg(not(feature = "async"))]
use embedded_hal::delay::DelayNs;
#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::delay::DelayNs;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

use crate::primitives::Address;
const RESPONSE_LEN: usize = 6;
/// Driver for STH4x sensors.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Sht4x<I, D> {
    i2c: I,
    address: Address,
    // If we want to globally define the delay type for this struct, we have to consume the type
    // parameter.
    _delay: PhantomData<D>,
}

impl<I, D> Sht4x<I, D>
where
    I: I2c,
    D: DelayNs,
{
    /// Creates a new driver instance using the given I2C bus. It configures the
    /// default I2C address 0x44 used by most family members.
    pub const fn new(i2c: I) -> Self { Self::new_with_address(i2c, Address::Address0x44) }

    /// Crates a new driver instance using the given I2C bus and address. This
    /// constructor allows to instantiate the driver for the SHT40-BD1B
    /// which uses the non-default I2C address 0x45.
    pub const fn new_with_address(i2c: I, address: Address) -> Self {
        Self {
            i2c,
            address,
            _delay: PhantomData,
        }
    }

    /// Destroys the driver and returns the used I2C bus.
    pub fn destroy(self) -> I { self.i2c }
}
