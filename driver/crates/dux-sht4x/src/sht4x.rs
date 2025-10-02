use core::marker::PhantomData;

#[cfg(not(feature = "async"))]
use embedded_hal::delay::DelayNs;
#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::delay::DelayNs;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;
#[cfg(not(feature = "async"))]
use sensirion_i2c::i2c;
#[cfg(feature = "async")]
use sensirion_i2c::i2c_async;

use crate::error::Sht4xError;
use crate::primitives::{Address, Command, HeatingDuration, HeatingPower, Measurement, Precision};
use crate::responses::{RESPONSE_LEN, sensor_data_from_response, serial_number_from_response};

/// Driver for STH4x sensors.
#[derive(Debug, Eq, Hash, PartialEq)]
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
    pub fn new(i2c: I) -> Self { Self::new_with_address(i2c, Address::Address0x44) }

    /// Crates a new driver instance using the given I2C bus and address. This
    /// constructor allows to instantiate the driver for the SHT40-BD1B
    /// which uses the non-default I2C address 0x45.
    pub fn new_with_address(i2c: I, address: Address) -> Self {
        Sht4x {
            i2c,
            address,
            _delay: PhantomData,
        }
    }

    /// Destroys the driver and returns the used I2C bus.
    pub fn destroy(self) -> I { self.i2c }
}
#[cfg(not(feature = "async"))]
impl<I, D> Sht4x<I, D>
where
    I: I2c,
    D: DelayNs,
{
    /// Performs a measurement returning measurands in SI units.
    pub fn measure(
        &mut self,
        precision: Precision,
        delay: &mut D,
    ) -> Result<Measurement, Sht4xError<I::Error>> {
        let command = Command::from(precision);

        self.send_command(command, delay)?;
        let response = self.read_response()?;
        let raw = sensor_data_from_response(response);

        Ok(Measurement::from(raw))
    }
    /// Activates the heater and performs a measurement returning measurands in
    /// SI units.
    ///
    /// **Note:** The heater is designed to be used up to 10 % of the sensor's
    /// lifetime.
    pub fn heat_and_measure(
        &mut self,
        power: HeatingPower,
        duration: HeatingDuration,
        delay: &mut D,
    ) -> Result<Measurement, Sht4xError<I::Error>> {
        let command = Command::from((power, duration));

        self.send_command(command, delay)?;
        let response = self.read_response()?;
        let raw = sensor_data_from_response(response);

        Ok(Measurement::from(raw))
    }

    /// Reads the sensor's serial number.
    pub fn serial_number(&mut self, delay: &mut D) -> Result<u32, Sht4xError<I::Error>> {
        self.send_command(Command::SerialNumber, delay)?;
        let response = self.read_response()?;
        Ok(serial_number_from_response(response))
    }

    /// Performs a soft reset of the sensor.
    pub fn soft_reset(&mut self, delay: &mut D) -> Result<(), Sht4xError<I::Error>> {
        self.send_command(Command::SoftReset, delay)
    }

    fn send_command(
        &mut self,
        command: Command,
        delay: &mut D,
    ) -> Result<(), Sht4xError<I::Error>> {
        let code = command.code();

        i2c::write_command_u8(&mut self.i2c, self.address.into(), code)
            .map_err(Sht4xError::I2cWrite)?;
        delay.delay_ms(command.duration_ms());

        Ok(())
    }

    fn read_response(&mut self) -> Result<[u8; RESPONSE_LEN], Sht4xError<I::Error>> {
        let mut response = [0; RESPONSE_LEN];

        i2c::read_words_with_crc(&mut self.i2c, self.address.into(), &mut response)?;

        Ok(response)
    }
}
#[cfg(feature = "async")]
impl<I, D> Sht4x<I, D>
where
    I: I2c,
    D: DelayNs,
{
    /// Performs a measurement returning measurands in SI units.
    pub async fn measure(
        &mut self,
        precision: Precision,
        delay: &mut D,
    ) -> Result<Measurement, Sht4xError<I::Error>> {
        let command = Command::from(precision);

        self.send_command(command, delay).await?;
        let response = self.read_response().await?;
        let raw = sensor_data_from_response(response);

        Ok(Measurement::from(raw))
    }
    /// Activates the heater and performs a measurement returning measurands in
    /// SI units.
    ///
    /// **Note:** The heater is designed to be used up to 10 % of the sensor's
    /// lifetime.
    pub async fn heat_and_measure(
        &mut self,
        power: HeatingPower,
        duration: HeatingDuration,
        delay: &mut D,
    ) -> Result<Measurement, Sht4xError<I::Error>> {
        let command = Command::from((power, duration));

        self.send_command(command, delay).await?;
        let response = self.read_response().await?;
        let raw = sensor_data_from_response(response);

        Ok(Measurement::from(raw))
    }

    /// Reads the sensor's serial number.
    pub async fn serial_number(&mut self, delay: &mut D) -> Result<u32, Sht4xError<I::Error>> {
        self.send_command(Command::SerialNumber, delay).await?;
        let response = self.read_response().await?;
        Ok(serial_number_from_response(response))
    }

    /// Performs a soft reset of the sensor.
    pub async fn soft_reset(&mut self, delay: &mut D) -> Result<(), Sht4xError<I::Error>> {
        self.send_command(Command::SoftReset, delay).await
    }

    async fn send_command(
        &mut self,
        command: Command,
        delay: &mut D,
    ) -> Result<(), Sht4xError<I::Error>> {
        let code = command.code();

        i2c_async::write_command_u8(&mut self.i2c, self.address.into(), code)
            .await
            .map_err(Sht4xError::I2cWrite)?;
        delay.delay_ms(command.duration_ms()).await;

        Ok(())
    }

    async fn read_response(&mut self) -> Result<[u8; RESPONSE_LEN], Sht4xError<I::Error>> {
        let mut response = [0; RESPONSE_LEN];

        i2c_async::read_words_with_crc(&mut self.i2c, self.address.into(), &mut response).await?;

        Ok(response)
    }
}
