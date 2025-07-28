#![windows_subsystem = "windows"]
mod models;
mod utils;
mod services;
mod gui;

use gui::App;
use std::path::Path;
use egui::IconData;

fn main() -> Result<(), eframe::Error> {
    // Load the icon for the window
    let icon_path = Path::new("favicon.ico");
    let icon = if icon_path.exists() {
        match load_icon(icon_path) {
            Ok(icon) => Some(icon),
            Err(_) => None,
        }
    } else {
        None
    };

    let mut viewport_builder = egui::ViewportBuilder::default()
        .with_inner_size([1200.0, 800.0])
        .with_min_inner_size([800.0, 600.0])
        .with_title("Olympix Pattern Tool - Solidity Security Scanner");
    
    // Set the window icon if loaded successfully
    if let Some(icon) = icon {
        viewport_builder = viewport_builder.with_icon(icon);
    }

    let options = eframe::NativeOptions {
        viewport: viewport_builder,
        ..Default::default()
    };
    
    eframe::run_native(
        "Olympix Pattern Tool",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}

fn load_icon(path: &Path) -> Result<IconData, Box<dyn std::error::Error>> {
    let image = image::load_from_memory(&std::fs::read(path)?)?;
    let rgba = image.to_rgba8();
    let size = [rgba.width() as _, rgba.height() as _];
    let pixels = rgba.into_raw();
    Ok(IconData { rgba: pixels, width: size[0], height: size[1] })
}
