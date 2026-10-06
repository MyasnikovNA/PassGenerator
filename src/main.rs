#![cfg_attr(windows, windows_subsystem = "windows")]

mod ui;
mod generator;


fn main() -> eframe::Result<()> {
    let window_size = [400.0, 250.0];
    
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Password Generator")
            .with_inner_size(window_size)
            .with_resizable(false),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Password Generator",
        native_options,
        Box::new(|cc| Ok(Box::new(ui::default::GenPassApp::new(cc)))),
    )
}