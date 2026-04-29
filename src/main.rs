#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod downloader;

use app::YtdwtApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([600.0, 450.0])
            .with_min_inner_size([500.0, 400.0])
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "ytdwt - YouTube Downloader",
        options,
        Box::new(|cc| Ok(Box::new(YtdwtApp::new(cc)))),
    )
}

fn load_icon() -> eframe::egui::IconData {
    let icon_bytes = include_bytes!("../app_icon.png");
    let image = image::load_from_memory(icon_bytes).expect("Failed to load icon");
    let image = image.to_rgba8();
    let (width, height) = image.dimensions();
    
    eframe::egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}
