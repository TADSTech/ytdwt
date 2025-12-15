use crate::config::{Config, DownloadFormat};
use crate::downloader::{DownloadOptions, DownloadState, Downloader};
use eframe::egui;
use std::sync::{Arc, Mutex};

pub struct YtdwtApp {
    config: Config,
    url: String,
    output_dir: String,
    quality: String,
    format: DownloadFormat,
    is_playlist: bool,
    downloader: Arc<Mutex<Downloader>>,
    runtime: tokio::runtime::Runtime,
}

impl Default for YtdwtApp {
    fn default() -> Self {
        let config = Config::load();
        let output_dir = config.default_output_dir.clone();
        let quality = config.default_quality.clone();
        let format = config.default_format;

        Self {
            config,
            url: String::new(),
            output_dir,
            quality,
            format,
            is_playlist: false,
            downloader: Arc::new(Mutex::new(Downloader::new())),
            runtime: tokio::runtime::Runtime::new().unwrap(),
        }
    }
}

impl YtdwtApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn start_download(&mut self) {
        if self.url.trim().is_empty() {
            return;
        }

        let options = DownloadOptions {
            url: self.url.clone(),
            output_dir: self.output_dir.clone(),
            quality: self.quality.clone(),
            audio_only: self.format == DownloadFormat::AudioOnly,
            is_playlist: self.is_playlist,
        };

        let downloader = Arc::clone(&self.downloader);
        self.runtime.spawn(async move {
            let _ = crate::downloader::run_download(downloader, options).await;
        });
    }

    fn select_output_dir(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.output_dir = path.to_string_lossy().to_string();
        }
    }

    fn save_config(&mut self) {
        self.config.default_output_dir = self.output_dir.clone();
        self.config.default_quality = self.quality.clone();
        self.config.default_format = self.format;
        let _ = self.config.save();
    }
}

impl eframe::App for YtdwtApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request continuous repaint for progress updates
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎬 ytdwt - YouTube Downloader with Tools");
            ui.add_space(10.0);

            // URL Input
            ui.horizontal(|ui| {
                ui.label("URL:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.url)
                        .hint_text("Enter YouTube URL or video ID")
                        .desired_width(400.0),
                );
            });
            ui.add_space(10.0);

            // Output Directory
            ui.horizontal(|ui| {
                ui.label("Output:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.output_dir)
                        .desired_width(300.0),
                );
                if ui.button("📁 Browse").clicked() {
                    self.select_output_dir();
                }
            });
            ui.add_space(10.0);

            // Format Selection
            ui.horizontal(|ui| {
                ui.label("Format:");
                ui.radio_value(&mut self.format, DownloadFormat::Video, "Video");
                ui.radio_value(&mut self.format, DownloadFormat::AudioOnly, "Audio Only (MP3)");
            });
            ui.add_space(5.0);

            // Quality Selection (only for video)
            if self.format == DownloadFormat::Video {
                ui.horizontal(|ui| {
                    ui.label("Quality:");
                    egui::ComboBox::from_id_salt("quality_selector")
                        .selected_text(&self.quality)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.quality, "best".to_string(), "Best");
                            ui.selectable_value(&mut self.quality, "1080p".to_string(), "1080p");
                            ui.selectable_value(&mut self.quality, "720p".to_string(), "720p");
                            ui.selectable_value(&mut self.quality, "480p".to_string(), "480p");
                        });
                });
                ui.add_space(5.0);
            }

            // Playlist Option
            ui.checkbox(&mut self.is_playlist, "Download entire playlist");
            ui.add_space(10.0);

            ui.separator();
            ui.add_space(10.0);

            // Download Button and Status
            let downloader = self.downloader.lock().unwrap();
            let can_download = matches!(downloader.state, DownloadState::Idle | DownloadState::Complete | DownloadState::Error(_));
            drop(downloader);

            ui.horizontal(|ui| {
                if ui
                    .add_enabled(can_download, egui::Button::new("⬇ Download"))
                    .clicked()
                {
                    self.start_download();
                }

                if ui.button("💾 Save Settings").clicked() {
                    self.save_config();
                }
            });
            ui.add_space(10.0);

            // Progress and Status
            let downloader = self.downloader.lock().unwrap();
            match &downloader.state {
                DownloadState::Idle => {
                    ui.label("Ready to download");
                }
                DownloadState::Downloading => {
                    ui.label(format!("Progress: {:.1}%", downloader.progress));
                    ui.add(egui::ProgressBar::new(downloader.progress / 100.0).show_percentage());
                    ui.label(&downloader.status_message);
                }
                DownloadState::Complete => {
                    ui.colored_label(egui::Color32::GREEN, "✓ Download complete!");
                }
                DownloadState::Error(err) => {
                    ui.colored_label(egui::Color32::RED, format!("✗ Error: {}", err));
                }
            }
            ui.add_space(20.0);

            // Footer
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Powered by yt-dlp");
                ui.hyperlink_to("ℹ Help", "https://github.com/yt-dlp/yt-dlp");
            });
        });
    }
}
