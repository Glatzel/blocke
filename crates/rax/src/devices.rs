//! Device enumeration and filtering utilities for serial devices.
//!
//! This module provides types and functions to list and filter serial devices
//! (such as USB, PCI, and Bluetooth devices) using the `serialport` crate.

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
    /// Create a new `DeviceInfo` instance.
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
    /// Get the device name.
    pub fn name(&self) -> &str { &self.name }
    /// Get the device type.
    pub fn device_type(&self) -> &DeviceType { &self.device_type }
    /// Get the vendor ID, if available.
    pub fn vendor_id(&self) -> Option<u16> { self.vendor_id }
    /// Get the product ID, if available.
    pub fn product_id(&self) -> Option<u16> { self.product_id }
    /// Get the serial number, if available.
    pub fn serial_number(&self) -> Option<&str> { self.serial_number.as_deref() }
    /// Get the manufacturer, if available.
    pub fn manufacturer(&self) -> Option<&str> { self.manufacturer.as_deref() }
    /// Get the product name, if available.
    pub fn product(&self) -> Option<&str> { self.product.as_deref() }
}

/// List available devices that match a custom filter.
///
/// # Arguments
/// * `filter` - A closure that takes a reference to [`SerialPortInfo`] and
///   returns `true` if the device should be included.
///
/// # Returns
/// * `Ok(Vec<DeviceInfo>)` on success.
/// * `Err(miette::Report)` if device enumeration fails.
pub fn list_devices<F>(filter: F) -> miette::Result<Vec<DeviceInfo>>
where
    F: Fn(&SerialPortInfo) -> bool,
{
    let ports = serialport::available_ports().into_diagnostic()?;
    // Log the number of ports found before filtering
    clerk::info!(
        "[Device] Found {} serial ports before filtering",
        ports.len()
    );
    let filtered_ports: Vec<_> = ports.into_iter().filter(&filter).collect();
    // Log the number of ports after filtering
    clerk::info!(
        "[Device] {} serial ports after filtering",
        filtered_ports.len()
    );
    // Map filtered SerialPortInfo into DeviceInfo, logging each port type
    Ok(filtered_ports
        .into_iter()
        .map(|p| match p.port_type {
            SerialPortType::UsbPort(info) => {
                // Log USB port details
                clerk::debug!("[Device] USB port: {:?}", p.port_name);
                DeviceInfo::new(
                    p.port_name,
                    DeviceType::Usb,
                    Some(info.vid),
                    Some(info.pid),
                    info.serial_number,
                    info.manufacturer,
                    info.product,
                )
            }
            SerialPortType::PciPort => {
                // Log PCI port details
                clerk::debug!("[Device] PCI port: {:?}", p.port_name);
                DeviceInfo::new(p.port_name, DeviceType::Pci, None, None, None, None, None)
            }
            SerialPortType::BluetoothPort => {
                // Log Bluetooth port details
                clerk::debug!("[Device] Bluetooth port: {:?}", p.port_name);
                DeviceInfo::new(
                    p.port_name,
                    DeviceType::Bluetooth,
                    None,
                    None,
                    None,
                    None,
                    None,
                )
            }
            SerialPortType::Unknown => {
                // Log unknown port details
                clerk::debug!("[Device] Unknown port: {:?}", p.port_name);
                DeviceInfo::new(
                    p.port_name,
                    DeviceType::Unknown,
                    None,
                    None,
                    None,
                    None,
                    None,
                )
            }
        })
        .collect())
}

/// Utility struct for common device filters.
pub struct DeviceFilter();
impl DeviceFilter {
    /// Filter that matches all devices.
    pub fn all(_: &SerialPortInfo) -> bool { true }
    /// Filter that matches only USB devices.
    pub fn usb(info: &SerialPortInfo) -> bool {
        matches!(info.port_type, SerialPortType::UsbPort { .. })
    }
}
