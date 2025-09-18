mod ui;
mod erasure;
mod verify;
mod cert;
mod utils;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Secure Wipe Prototype",
        options,
        Box::new(|_cc| Ok(Box::new(ui::SecureWipeApp::default()))),
    )
}