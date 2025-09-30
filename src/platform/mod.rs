use crate::erasure::ata::{AtaDeviceInfo, AtaError, AtaStorageDriver};

#[cfg(target_os = "linux")]
pub mod linux;

#[derive(Debug)]
pub struct StorageDevice {
    pub path: String,
    pub name: String,
    pub size: Option<u64>,
    pub device_type: DeviceType,
    pub serial_number: Option<String>,
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub status: DeviceStatus,
}

#[derive(Debug)]
pub enum DeviceStatus {
    Active,
    Mounted,
    Available,
    // Unknown,
}

#[derive(Debug)]
pub enum DeviceType {
    HDD,
    SSD,
    NVMe,
    USB,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WipeMethod {
    Overwrite,
    SecureErase,
    CryptoErase,
}

impl std::fmt::Display for WipeMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WipeMethod::Overwrite => write!(f, "Overwrite (Multiple Pass)"),
            WipeMethod::SecureErase => write!(f, "Secure Erase (ATA/NVMe)"),
            WipeMethod::CryptoErase => write!(f, "Crypto Erase (SED)"),
        }
    }
}

impl WipeMethod {
    pub fn get_all_methods() -> Vec<WipeMethod> {
        vec![
            WipeMethod::Overwrite,
            WipeMethod::SecureErase,
            WipeMethod::CryptoErase,
        ]
    }
}

pub trait StorageInterface {
    fn detect_storage_devices() -> Vec<StorageDevice>;
    fn format_size(bytes: u64) -> String;

    fn identify_ata_device(ata_storage_driver: &AtaStorageDriver) -> Result<AtaDeviceInfo, AtaError>;
}