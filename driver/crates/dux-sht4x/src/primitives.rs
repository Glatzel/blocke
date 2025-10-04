use fixed::types::{I16F16, U16F16};
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum Command {
    MeasureHighPrecision = 0xfd,
    MeasureMediumPrecision = 0xf6,
    MeasureLowPrecision = 0xe0,
    SoftReset = 0x89,
    SerialNumber = 0x94,
    MeasureHeated200mw1s = 0x39,
    MeasureHeated200mw0p1s = 0x32,
    MeasureHeated110mw1s = 0x2f,
    MeasureHeated110mw0p1s = 0x24,
    MeasureHeated20mw1s = 0x1e,
    MeasureHeated20mw0p1s = 0x15,
}

impl Command {
    pub(crate) fn duration_ms(&self) -> u32 {
        // Values rounded up from the maximum durations given in the datasheet
        // table 4, 'System timing specifications'.
        match self {
            Self::MeasureHighPrecision => 9,
            Self::MeasureMediumPrecision => 5,
            Self::MeasureLowPrecision => 2,
            // There is no explicit time given for the serial number, but reading it immediately
            // results in a NACK. So be a bit more patient here.
            Self::SerialNumber => 1,
            Self::SoftReset => 1,
            Self::MeasureHeated200mw1s => 1100,
            Self::MeasureHeated200mw0p1s => 110,
            Self::MeasureHeated110mw1s => 1100,
            Self::MeasureHeated110mw0p1s => 110,
            Self::MeasureHeated20mw1s => 1100,
            Self::MeasureHeated20mw0p1s => 110,
        }
    }
}
impl From<(HeatingPower, HeatingDuration)> for Command {
    fn from((power, duration): (HeatingPower, HeatingDuration)) -> Self {
        match (power, duration) {
            (HeatingPower::Low, HeatingDuration::Short) => Command::MeasureHeated20mw0p1s,
            (HeatingPower::Low, HeatingDuration::Long) => Command::MeasureHeated20mw1s,
            (HeatingPower::Medium, HeatingDuration::Short) => Command::MeasureHeated110mw0p1s,
            (HeatingPower::Medium, HeatingDuration::Long) => Command::MeasureHeated110mw1s,
            (HeatingPower::High, HeatingDuration::Short) => Command::MeasureHeated200mw0p1s,
            (HeatingPower::High, HeatingDuration::Long) => Command::MeasureHeated200mw1s,
        }
    }
}

impl From<Precision> for Command {
    fn from(precision: Precision) -> Self {
        match precision {
            Precision::Low => Command::MeasureLowPrecision,
            Precision::Medium => Command::MeasureMediumPrecision,
            Precision::High => Command::MeasureHighPrecision,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Address {
    Address0x44 = 0x44,
    Address0x45 = 0x45,
    Address0x46 = 0x46,
}

/// Heating power to apply when activating the internal heater.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HeatingPower {
    /// Operate the heater at 200 mW.
    Low,
    /// Operate the heater at 110 mW.
    Medium,
    /// Operate the heater at 20 mW.
    High,
}
/// Duration of heating when activating the internal heater.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HeatingDuration {
    /// Operate the heater for 100 ms.
    Short,
    /// Operate the heater for 1 s.
    Long,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Precision {
    Low,
    Medium,
    High,
}
/// A measurement from the sensor in SI units.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Measurement {
    /// The measurred temperature in degree Celsius (°C).
    temperature: I16F16,
    /// The measured relative humidity in percent (%).
    humidity: I16F16,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct SensorData {
    /// The measured temperature as raw sensor value.
    pub temperature: u16,
    /// The measured relative humidity as raw sensor value.
    pub humidity: u16,
}
impl From<SensorData> for Measurement {
    /// Converts raw sensor data into SI units.
    fn from(raw: SensorData) -> Self {
        const MINUS_45: I16F16 = I16F16::const_from_int(-45);
        const MINUS_6: I16F16 = I16F16::const_from_int(-6);

        let temperature_quotient = U16F16::from_num(raw.temperature) / (u16::MAX as u32);
        let humidity_quotient = U16F16::from_num(raw.humidity) / (u16::MAX as u32);

        Self {
            temperature: MINUS_45 + 175 * temperature_quotient.to_num::<I16F16>(),
            humidity: MINUS_6 + 125 * humidity_quotient.to_num::<I16F16>(),
        }
    }
}
impl Measurement {
    /// Returns the measured temperature in degree Celsius (°C).
    pub fn temperature_celsius(&self) -> I16F16 { self.temperature }

    /// Returns the measured relative humidity in percent (%).
    pub fn humidity_percent(&self) -> I16F16 { self.humidity }
}
#[cfg(test)]
mod tests {
    extern crate std;
    use std::string::ToString;

    use super::*;

    #[test]
    fn test_sensor_data_conversion() {
        // Edge cases
        let raw_min = SensorData {
            temperature: 0,
            humidity: 0,
        };
        let raw_max = SensorData {
            temperature: u16::MAX,
            humidity: u16::MAX,
        };
        let raw_mid = SensorData {
            temperature: u16::MAX / 2,
            humidity: u16::MAX / 2,
        };

        let meas_min = Measurement::from(raw_min);
        let meas_max = Measurement::from(raw_max);
        let meas_mid = Measurement::from(raw_mid);

        assert_eq!(meas_min.temperature_celsius().to_string(), "-45");
        assert_eq!(meas_max.temperature_celsius().to_string(), "130");
        assert_eq!(meas_mid.temperature_celsius().to_string(), "42.49733");

        assert_eq!(meas_min.humidity_percent().to_string(), "-6");
        assert_eq!(meas_max.humidity_percent().to_string(), "119");
        assert_eq!(meas_mid.humidity_percent().to_string(), "56.4981");
    }
}
