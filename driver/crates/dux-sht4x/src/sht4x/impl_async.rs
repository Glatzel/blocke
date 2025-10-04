use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::{Error, I2c};
use sensirion_i2c::i2c_async;

use crate::error::Sht4xError;
use crate::sht4x::RESPONSE_LEN;
use crate::{Command, HeatingDuration, HeatingPower, Measurement, Precision, SensorData, Sht4x};

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
    ) -> Result<Measurement, Sht4xError> {
        let command = Command::from(precision);

        self.send_command(command, delay).await?;
        let response = self.read_response().await?;
        let raw = SensorData {
            temperature: u16::from_be_bytes([response[0], response[1]]),
            humidity: u16::from_be_bytes([response[3], response[4]]),
        };

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
    ) -> Result<Measurement, Sht4xError> {
        let command = Command::from((power, duration));

        self.send_command(command, delay).await?;
        let response = self.read_response().await?;
        let raw = SensorData {
            temperature: u16::from_be_bytes([response[0], response[1]]),
            humidity: u16::from_be_bytes([response[3], response[4]]),
        };

        Ok(Measurement::from(raw))
    }

    /// Reads the sensor's serial number.
    pub async fn serial_number(&mut self, delay: &mut D) -> Result<u32, Sht4xError> {
        self.send_command(Command::SerialNumber, delay).await?;
        let response = self.read_response().await?;
        Ok(u32::from_be_bytes([
            response[0],
            response[1],
            response[3],
            response[4],
        ]))
    }

    /// Performs a soft reset of the sensor.
    pub async fn soft_reset(&mut self, delay: &mut D) -> Result<(), Sht4xError> {
        self.send_command(Command::SoftReset, delay).await
    }

    async fn send_command(&mut self, command: Command, delay: &mut D) -> Result<(), Sht4xError> {
        i2c_async::write_command_u8(&mut self.i2c, self.address as u8, command as u8)
            .await
            .map_err(|e| Sht4xError::I2cWrite(e.kind()))?;
        delay.delay_ms(command.duration_ms()).await;

        Ok(())
    }

    async fn read_response(&mut self) -> Result<[u8; RESPONSE_LEN], Sht4xError> {
        let mut response = [0; RESPONSE_LEN];

        i2c_async::read_words_with_crc(&mut self.i2c, self.address as u8, &mut response).await?;

        Ok(response)
    }
}
