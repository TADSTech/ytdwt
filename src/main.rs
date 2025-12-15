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
    // Create a simple icon (16x16 red square as placeholder)
    let icon_size = 16;
    let mut rgba = vec![0u8; icon_size * icon_size * 4];
    
    for i in 0..icon_size * icon_size {
        let offset = i * 4;
        rgba[offset] = 220;     // R
        rgba[offset + 1] = 38;  // G
        rgba[offset + 2] = 38;  // B
        rgba[offset + 3] = 255; // A
    }

    eframe::egui::IconData {
        rgba,
        width: icon_size as u32,
        height: icon_size as u32,
    }
}
