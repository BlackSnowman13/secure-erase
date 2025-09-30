use std::{fs::{self, OpenOptions}, os::fd::AsRawFd};

use crate::platform::{StorageInterface};

pub struct AtaDeviceInfo {
    pub model: String,
    pub serial: String,
    pub firmware: String,
    pub sector_size: u16,
    pub sector_count: u64,
    pub security_supported: bool,
    pub security_enabled: bool,
    pub security_locked: bool,
    pub security_frozen: bool,
    pub enhanced_erase_supported: bool,
}

#[derive(Debug)]
pub enum AtaError {
    IoError(std::io::Error),
    Unsupported,
    DeviceNotFound,
    PermissionDenied,
    CommandFailed(String),
}

pub struct AtaStorageDriver {
	// TODO: Add fields for device path, file descriptor, etc.
    device_path: String,
    fd: i32,
}

impl AtaStorageDriver {
	pub fn new(device_path: &str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(device_path)
            .expect("Failed to open device");

        let fd = file.as_raw_fd();
        AtaStorageDriver {
            device_path: device_path.to_string(),
            fd,
        }
    }

    pub fn is_ata_device(&self) -> bool {
        // Check if the device is an ATA device
        let device_path = format!("/sys/block/{}/device", self.device_path);
        if let Ok(content) = fs::read_to_string(&device_path) {
            return content.contains("ata");
        }
        false
    }

    pub fn identify_ata_device(&self) -> Result<AtaDeviceInfo, AtaError> {
        #[cfg(target_os = "linux")]
        {
            crate::platform::linux::LinuxStorage::identify_ata_device(self)
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(AtaError::Unsupported)
        }
    }
}

// TODO: Add ATA command interface
// TODO: Add hdparm wrapper functions
// TODO: Add direct ioctl implementation as fallback
// TODO: Add ATA device identification
// TODO: Add secure erase unit command
// TODO: Add enhanced secure erase command
// TODO: Add time estimation for erase operations
// TODO: Add password handling for frozen drives
// TODO: Add error handling for ATA-specific errors
// TODO: Add progress monitoring for long operations