use crate::erasure::ata::{AtaStorageDriver};

mod ui;
mod erasure;
mod verify;
mod cert;
mod platform;
mod libs;

fn main() -> eframe::Result<()> {
    // is_ata_secure_erase_available("/dev/sdc".to_string());
    let ata_driver = AtaStorageDriver::new("/dev/nvme0n1");
    match ata_driver.is_ata_device() {
        true => {
            println!("Secure Erase is supported on this ATA device.");
            // match ata_driver.identify_ata_device() {
            //     Ok(info) => {
            //         println!("Model: {}", info.model);
            //         println!("Serial: {}", info.serial);
            //         println!("Firmware: {}", info.firmware);
            //         println!("Sector Size: {} bytes", info.sector_size);
            //         println!("Sector Count: {}", info.sector_count);
            //         println!("Security Supported: {}", info.security_supported);
            //         println!("Security Enabled: {}", info.security_enabled);
            //         println!("Security Locked: {}", info.security_locked);
            //         println!("Security Frozen: {}", info.security_frozen);
            //         println!("Enhanced Erase Supported: {}", info.enhanced_erase_supported);
            //     },
            //     Err(e) => {
            //         eprintln!("Failed to identify ATA device: {:?}", e);
            //     }
                
            // }
        },
        false => println!("Secure Erase is NOT supported on this ATA device."),
    }
    match ata_driver.identify_ata_device() {
        Ok(info) => {
            println!("Model: {}", info.model);
            println!("Serial: {}", info.serial);
            println!("Firmware: {}", info.firmware);
            println!("Sector Size: {} bytes", info.sector_size);
            println!("Sector Count: {}", info.sector_count);
            println!("Security Supported: {}", info.security_supported);
            println!("Security Enabled: {}", info.security_enabled);
            println!("Security Locked: {}", info.security_locked);
            println!("Security Frozen: {}", info.security_frozen);
            println!("Enhanced Erase Supported: {}", info.enhanced_erase_supported);
        },
        Err(e) => {
            eprintln!("Failed to identify ATA device: {:?}", e);
        }
        
    }
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Secure Wipe",
        options,
        Box::new(|cc| Ok(Box::new(ui::SecureWipeApp::new(cc)))),
        
    )
}