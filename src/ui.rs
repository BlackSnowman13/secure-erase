use egui::{self, Align, Layout};
use crate::utils::{detect_storage_devices, StorageDevice};

pub struct SecureWipeApp {
    // Storage device management
    available_devices: Vec<StorageDevice>,
    selected_device_index: usize,
}

impl Default for SecureWipeApp {
    fn default() -> Self {
        let mut app = Self {
            available_devices: Vec::new(),
            selected_device_index: 0,
        };
        app.refresh_devices();
        app
    }
}

impl eframe::App for SecureWipeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Centered heading
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(20.0);
                ui.heading("Secure Wipe");
                ui.add_space(30.0);
            });

            // Storage device selection section
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Storage Device:");
                    
                    // Device dropdown
                    let selected_text = if self.available_devices.is_empty() {
                        "No devices detected".to_string()
                    } else {
                        self.available_devices[self.selected_device_index].to_string()
                    };
                    
                    egui::ComboBox::from_id_salt("device_selector")
                        .selected_text(&selected_text)
                        .show_ui(ui, |ui| {
                            for (index, device) in self.available_devices.iter().enumerate() {
                                ui.selectable_value(&mut self.selected_device_index, index, device.to_string());
                            }
                        });
                    
                    // Refresh button
                    if ui.button("🔄 Refresh").clicked() {
                        // TODO: Implement device refresh logic
                        self.refresh_devices();
                    }
                });
                
                ui.add_space(20.0);
                
                // Wipe Data button
                let wipe_button_enabled = !self.available_devices.is_empty();
                ui.add_enabled_ui(wipe_button_enabled, |ui| {
                    if ui.button("🗑️ Wipe Data").clicked() {
                        // TODO: Implement wipe data logic
                        // For now, just show that button was clicked
                        if let Some(device) = self.available_devices.get(self.selected_device_index) {
                            println!("Wipe Data button clicked for device: {}", device);
                        }
                    }
                });
            });

            // TODO: Add progress bar here when wiping is in progress
            // TODO: Add erasure method selection
            // TODO: Add verification options
            // TODO: Add certificate generation options
        });

        // request a redraw for UI updates
        ctx.request_repaint();
    }
}

impl SecureWipeApp {
    fn refresh_devices(&mut self) {
        // Use actual device detection
        self.available_devices = detect_storage_devices();
        
        // Reset selection to first device or 0 if no devices
        self.selected_device_index = 0;
        
        // Ensure selected index is valid
        if self.selected_device_index >= self.available_devices.len() && !self.available_devices.is_empty() {
            self.selected_device_index = 0;
        }
    }
}