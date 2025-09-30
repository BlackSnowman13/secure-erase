// Storage Driver - Cross-platform storage device interface
use crate::platform::{StorageDevice, StorageInterface};

pub struct StorageDriver;

impl StorageDriver {
    pub fn detect_storage_devices() -> Vec<StorageDevice> {
        #[cfg(target_os = "linux")]
        {
            crate::platform::linux::LinuxStorage::detect_storage_devices()
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            Vec::new()
        }
    }

    pub fn format_size(bytes: u64) -> String {
        #[cfg(target_os = "linux")]
        {
            crate::platform::linux::LinuxStorage::format_size(bytes)
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            format!("{} bytes", bytes)
        }
    }
}