use std::{fs, path::Path};
use crate::{erasure::ata::{AtaDeviceInfo, AtaError, AtaStorageDriver}, platform::{DeviceStatus, DeviceType, StorageDevice, StorageInterface}};

pub struct LinuxStorage;

impl StorageInterface for LinuxStorage {
    fn detect_storage_devices() -> Vec<super::StorageDevice> {
        let mut devices = Vec::new();
    
        // Check /sys/block for block devices
        if let Ok(entries) = fs::read_dir("/sys/block") {
            for entry in entries.flatten() {
                if let Some(device_name) = entry.file_name().to_str() {
                    // Skip loop devices, ram disks, and other virtual devices
                    if device_name.starts_with("loop") 
                        || device_name.starts_with("ram")
                        || device_name.starts_with("dm-") {
                        continue;
                    }
                    
                    let device_path = format!("/dev/{}", device_name);
                    
                    // Check if device file exists and is accessible
                    if Path::new(&device_path).exists() {
                        let device = create_storage_device(&device_path, device_name);
                        devices.push(device);
                    }
                }
            }
        }
        
        // Sort devices by path for consistent ordering
        devices.sort_by(|a, b| a.path.cmp(&b.path));
        
        devices
    }

    fn format_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        if unit_index == 0 {
            format!("{} {}", size as u64, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }


    fn identify_ata_device(ata_storage_driver: &AtaStorageDriver) -> Result<AtaDeviceInfo, AtaError> {
        println!("LinuxStorage::identify_ata_device called");
        
        todo!()
    }
}

fn create_storage_device(device_path: &str, device_name: &str) -> StorageDevice {
    let model = get_device_model(device_name);
    let name = model.clone().unwrap_or_else(|| device_name.to_string());
    let size = get_device_size(device_name);
    let device_type = determine_device_type(device_name, device_path);
    let serial_number = get_device_serial(device_name);
    let vendor = get_device_vendor(device_name);
    let status = get_device_status(device_name);
    
    StorageDevice {
        path: device_path.to_string(),
        name,
        size,
        device_type,
        serial_number,
        model,
        vendor,
        status,
    }
}

fn get_device_model(device_name: &str) -> Option<String> {
    // Try to read model information from sysfs
    let model_paths = [
        format!("/sys/block/{}/device/model", device_name),
        format!("/sys/block/{}/device/name", device_name),
        format!("/sys/block/{}/queue/rotational", device_name),
    ];
    
    for path in &model_paths {
        if let Ok(content) = fs::read_to_string(path) {
            let trimmed = content.trim();
            if !trimmed.is_empty() && trimmed != "0" && trimmed != "1" {
                return Some(trimmed.to_string());
            }
        }
    }
    
    None
}

fn get_device_size(device_name: &str) -> Option<u64> {
    let size_path = format!("/sys/block/{}/size", device_name);
    
    if let Ok(content) = fs::read_to_string(&size_path) {
        if let Ok(sectors) = content.trim().parse::<u64>() {
            // Convert sectors to bytes (assuming 512 bytes per sector)
            return Some(sectors * 512);
        }
    }
    
    None
}

fn determine_device_type(device_name: &str, _device_path: &str) -> DeviceType {
    // Check if it's an NVMe device
    if device_name.starts_with("nvme") {
        return DeviceType::NVMe;
    }
    
    // Check rotation to distinguish between SSD and HDD
    let rotational_path = format!("/sys/block/{}/queue/rotational", device_name);
    if let Ok(content) = fs::read_to_string(&rotational_path) {
        if content.trim() == "0" {
            return DeviceType::SSD;
        } else if content.trim() == "1" {
            return DeviceType::HDD;
        }
    }
    
    // Check if it's a USB device
    let device_path_sys = format!("/sys/block/{}/device", device_name);
    if let Ok(target) = fs::read_link(&device_path_sys) {
        if target.to_string_lossy().contains("usb") {
            return DeviceType::USB;
        }
    }
    
    DeviceType::Unknown
}

fn get_device_serial(device_name: &str) -> Option<String> {
    // Try to read serial number from sysfs
    let serial_paths = [
        format!("/sys/block/{}/device/serial", device_name),
        format!("/sys/block/{}/device/wwid", device_name),
    ];
    
    for path in &serial_paths {
        if let Ok(content) = fs::read_to_string(path) {
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    
    None
}

fn get_device_vendor(device_name: &str) -> Option<String> {
    // Try to read vendor information from sysfs
    let vendor_paths = [
        format!("/sys/block/{}/device/vendor", device_name),
        format!("/sys/block/{}/device/manufacturer", device_name),
    ];
    
    for path in &vendor_paths {
        if let Ok(content) = fs::read_to_string(path) {
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    
    None
}

fn get_device_status(device_name: &str) -> DeviceStatus {
    // Check if device is mounted
    if let Ok(mounts) = fs::read_to_string("/proc/mounts") {
        if mounts.contains(&format!("/dev/{}", device_name)) {
            return DeviceStatus::Mounted;
        }
    }
    
    // Check if device is available/active
    let state_path = format!("/sys/block/{}/device/state", device_name);
    if let Ok(content) = fs::read_to_string(&state_path) {
        match content.trim() {
            "running" | "active" => return DeviceStatus::Active,
            _ => return DeviceStatus::Available,
        }
    }
    
    DeviceStatus::Available
}