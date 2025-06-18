use miette::IntoDiagnostic;
use serialport::{SerialPortInfo, SerialPortType};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceType {
    Usb,
    Pci,
    Bluetooth,
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    name: String,
    device_type: DeviceType,
    vendor_id: Option<u16>,
    product_id: Option<u16>,
    serial_number: Option<String>,
    manufacturer: Option<String>,
    product: Option<String>,
}
impl DeviceInfo {
    pub fn new(
        name: String,
        device_type: DeviceType,
        vendor_id: Option<u16>,
        product_id: Option<u16>,
        serial_number: Option<String>,
        manufacturer: Option<String>,
        product: Option<String>,
    ) -> Self {
        Self {
            name,
            device_type,
            vendor_id,
            product_id,
            serial_number,
            manufacturer,
            product,
        }
    }
    pub fn name(&self) -> &str { &self.name }
    pub fn device_type(&self) -> &DeviceType { &self.device_type }
    pub fn vendor_id(&self) -> Option<u16> { self.vendor_id }
    pub fn product_id(&self) -> Option<u16> { self.product_id }
    pub fn serial_number(&self) -> Option<&str> { self.serial_number.as_deref() }
    pub fn manufacturer(&self) -> Option<&str> { self.manufacturer.as_deref() }
    pub fn product(&self) -> Option<&str> { self.product.as_deref() }
}

/// List available devices that match a custom filter.
pub fn list_devices<F>(filter: F) -> miette::Result<Vec<DeviceInfo>>
where
    F: Fn(&SerialPortInfo) -> bool,
{
    let ports = serialport::available_ports().into_diagnostic()?;
    Ok(ports
        .into_iter()
        .filter(filter)
        .map(|p| match p.port_type {
            SerialPortType::UsbPort(info) => DeviceInfo::new(
                p.port_name,
                DeviceType::Usb,
                Some(info.vid),
                Some(info.pid),
                info.serial_number,
                info.manufacturer,
                info.product,
            ),
            SerialPortType::PciPort => {
                DeviceInfo::new(p.port_name, DeviceType::Pci, None, None, None, None, None)
            }
            SerialPortType::BluetoothPort => DeviceInfo::new(
                p.port_name,
                DeviceType::Bluetooth,
                None,
                None,
                None,
                None,
                None,
            ),
            SerialPortType::Unknown => DeviceInfo::new(
                p.port_name,
                DeviceType::Unknown,
                None,
                None,
                None,
                None,
                None,
            ),
        })
        .collect())
}
pub struct DeviceFilter();
impl DeviceFilter {
    pub fn all(_: &SerialPortInfo) -> bool { true }
    pub fn usb(info: &SerialPortInfo) -> bool {
        matches!(info.port_type, SerialPortType::UsbPort { .. })
    }
}
