use crate::primitives::SensorData;

pub(crate) const RESPONSE_LEN: usize = 6;

pub(crate) fn sensor_data_from_response(response: [u8; RESPONSE_LEN]) -> SensorData {
    SensorData {
        temperature: u16::from_be_bytes([response[0], response[1]]),
        humidity: u16::from_be_bytes([response[3], response[4]]),
    }
}

pub(crate) fn serial_number_from_response(response: [u8; RESPONSE_LEN]) -> u32 {
    u32::from_be_bytes([response[0], response[1], response[3], response[4]])
}
