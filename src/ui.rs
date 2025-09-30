use egui::{self, Align, Color32, Layout, RichText, Vec2};

use crate::{platform::{StorageDevice, WipeMethod}};
use crate::libs::StorageDriver;

pub struct SecureWipeApp {
    // Storage device management
    available_devices: Vec<StorageDevice>,
    selected_device_index: Option<usize>,

    // Wipe method selection
    selected_wipe_method: WipeMethod,
}

impl  SecureWipeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_material_icons::initialize(&cc.egui_ctx);
        let mut app = Self {
            available_devices: Vec::new(),
            selected_device_index: None,
            selected_wipe_method: WipeMethod::Overwrite,
        };
        app.refresh_devices();
        app
    }
    
}

impl eframe::App for SecureWipeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set white background
        let mut visuals = ctx.style().visuals.clone();
        visuals.window_fill = Color32::WHITE;
        visuals.panel_fill = Color32::WHITE;
        visuals.extreme_bg_color = Color32::WHITE;
        ctx.set_visuals(visuals);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Header section with shield icon and title
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(20.0);

                // Header with shield icon and title
                ui.horizontal(|ui| {
                    ui.label(RichText::new(egui_material_icons::icons::ICON_SHIELD)
                        .size(32.0)
                    );
                    ui.add_space(5.0);
                    ui.label(
                        RichText::new("Secure Wipe")
                            .size(32.0)
                            .color(Color32::BLACK),
                    );
                });

                ui.add_space(10.0);

                // Description
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Detect, analyze, and securely wipe storage devices")
                            .size(16.0)
                            .color(Color32::DARK_GRAY),
                    );
                });

                ui.add_space(30.0);
            });

            // Main content area - two columns
            ui.horizontal(|ui| {
                // Left side - Device list
                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width() * 0.5, ui.available_height()),
                    Layout::top_down(Align::Min),
                    |ui| {
                        ui.group(|ui| {
                            ui.set_min_height(300.0);
                            ui.vertical(|ui| {
                                // Device list header with refresh button
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new("Storage Devices")
                                            .size(18.0)
                                            .color(Color32::BLACK)
                                            .strong(),
                                    );
                                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                        if ui.button("🔄 Refresh").clicked() {
                                            self.refresh_devices();
                                        }
                                    });
                                });

                                ui.separator();
                                ui.add_space(10.0);

                                // Device list
                                if self.available_devices.is_empty() {
                                    ui.label("No devices detected");
                                } else {
                                    for (index, device) in self.available_devices.iter().enumerate()
                                    {
                                        let is_selected = self.selected_device_index == Some(index);

                                        let response = ui.selectable_label(
                                            is_selected,
                                            format!(
                                                "💾 {}\n   {} | {:?}",
                                                device.path,
                                                device
                                                    .size
                                                    .map(|s| StorageDriver::format_size(s))
                                                    .unwrap_or_else(|| "Unknown size".to_string()),
                                                device.device_type
                                            ),
                                        );

                                        if response.clicked() {
                                            self.selected_device_index = Some(index);
                                        }

                                        ui.add_space(5.0);
                                    }
                                }
                            });
                        });
                    },
                );

                ui.add_space(20.0);

                // Right side - Device details and controls
                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width(), ui.available_height()),
                    Layout::top_down(Align::Min),
                    |ui| {
                        ui.group(|ui| {
                            ui.set_min_height(300.0);
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new("Device Information")
                                        .size(18.0)
                                        .color(Color32::BLACK)
                                        .strong(),
                                );
                                ui.separator();
                                ui.add_space(10.0);

                                if let Some(selected_index) = self.selected_device_index {
                                    if let Some(device) = self.available_devices.get(selected_index)
                                    {
                                        // Device details
                                        ui.label(format!(
                                            "📋 Model: {}",
                                            device.model.as_ref().unwrap_or(&"Unknown".to_string())
                                        ));
                                        ui.label(format!("🔧 Type: {:?}", device.device_type));
                                        ui.label(format!(
                                            "💿 Size: {}",
                                            device
                                                .size
                                                .map(|s| StorageDriver::format_size(s))
                                                .unwrap_or_else(|| "Unknown".to_string())
                                        ));
                                        ui.label(format!("📊 Status: {:?}", device.status));
                                        ui.label(format!("🏷️ Path: {}", device.path));
                                        ui.label(format!(
                                            "🔢 Serial: {}",
                                            device
                                                .serial_number
                                                .as_ref()
                                                .unwrap_or(&"Unknown".to_string())
                                        ));
                                        ui.label(format!(
                                            "🏢 Vendor: {}",
                                            device
                                                .vendor
                                                .as_ref()
                                                .unwrap_or(&"Unknown".to_string())
                                        ));

                                        ui.add_space(10.0);
                                        ui.separator();
                                        ui.add_space(10.0);

                                        // ATA Secure Erase Support Status
                                        ui.label(
                                            RichText::new("ATA Secure Erase Support")
                                                .size(16.0)
                                                .color(Color32::BLACK)
                                                .strong(),
                                        );
                                        ui.add_space(5.0);


                                        ui.add_space(20.0);

                                        // Wipe method selection
                                        ui.label(
                                            RichText::new("Wipe Method")
                                                .size(16.0)
                                                .color(Color32::BLACK)
                                                .strong(),
                                        );
                                        ui.add_space(5.0);

                                        egui::ComboBox::from_id_salt("wipe_method_selector")
                                            .selected_text(self.selected_wipe_method.to_string())
                                            .show_ui(ui, |ui| {
                                                for method in WipeMethod::get_all_methods() {
                                                    ui.selectable_value(
                                                        &mut self.selected_wipe_method,
                                                        method.clone(),
                                                        method.to_string(),
                                                    );
                                                }
                                            });
                                    }
                                } else {
                                    ui.label("Select a device to show device information");
                                }
                            });
                        });
                    },
                );
            });

            ui.add_space(30.0);

            // Start Erase button (centered at bottom)
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                let start_button_enabled = self.selected_device_index.is_some();
                ui.add_enabled_ui(start_button_enabled, |ui| {
                    if ui
                        .button(RichText::new("🗑️ Start Erase").size(18.0))
                        .clicked()
                    {
                        // TODO: Implement erase logic
                        if let Some(selected_index) = self.selected_device_index {
                            if let Some(device) = self.available_devices.get(selected_index) {
                                println!(
                                    "Start Erase clicked for device: {} with method: {}",
                                    device.name, self.selected_wipe_method
                                );
                            }
                        }
                    }
                });
            });
        });

        // request a redraw for UI updates
        ctx.request_repaint();
    }
}

impl SecureWipeApp {
    fn refresh_devices(&mut self) {
        // Use actual device detection
        self.available_devices = StorageDriver::detect_storage_devices();

        // Reset selection to first device if available
        if !self.available_devices.is_empty() {
            self.selected_device_index = Some(0);
        } else {
            self.selected_device_index = None;
        }
    }
}
