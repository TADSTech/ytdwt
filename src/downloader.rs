use std::process::Stdio;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum DownloadState {
    Idle,
    Downloading,
    Complete,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct DownloadOptions {
    pub url: String,
    pub output_dir: String,
    pub quality: String,
    pub audio_only: bool,
    pub is_playlist: bool,
}

pub struct Downloader {
    pub state: DownloadState,
    pub progress: f32,
    pub status_message: String,
}

impl Default for Downloader {
    fn default() -> Self {
        Self {
            state: DownloadState::Idle,
            progress: 0.0,
            status_message: String::new(),
        }
    }
}

impl Downloader {
    pub fn new() -> Self {
        Self::default()
    }

    fn parse_progress(&mut self, line: &str) {
        // Parse yt-dlp progress output
        if line.contains("[download]") {
            if line.contains("%") {
                // Extract percentage
                if let Some(percent_pos) = line.find('%') {
                    let start = line[..percent_pos]
                        .rfind(|c: char| !c.is_numeric() && c != '.')
                        .map(|i| i + 1)
                        .unwrap_or(0);
                    if let Ok(percent) = line[start..percent_pos].trim().parse::<f32>() {
                        self.progress = percent;
                    }
                }
            }
            self.status_message = line.to_string();
        } else if line.contains("Destination:") || line.contains("Merging") {
            self.status_message = line.to_string();
        }
    }
}

pub async fn run_download(downloader: Arc<Mutex<Downloader>>, options: DownloadOptions) -> Result<(), String> {
    // Set initial state
    {
        let mut dl = downloader.lock().unwrap();
        dl.state = DownloadState::Downloading;
        dl.progress = 0.0;
        dl.status_message = "Starting download...".to_string();
    }

    let mut args = vec![
        "-o".to_string(),
        format!("{}/%(title)s.%(ext)s", options.output_dir),
    ];

    // Quality selection
    if options.audio_only {
        args.push("-f".to_string());
        args.push("bestaudio".to_string());
        args.push("-x".to_string());
        args.push("--audio-format".to_string());
        args.push("mp3".to_string());
    } else {
        match options.quality.as_str() {
            "best" => {
                args.push("-f".to_string());
                args.push("bestvideo+bestaudio/best".to_string());
            }
            "1080p" => {
                args.push("-f".to_string());
                args.push("bestvideo[height<=1080]+bestaudio/best[height<=1080]".to_string());
            }
            "720p" => {
                args.push("-f".to_string());
                args.push("bestvideo[height<=720]+bestaudio/best[height<=720]".to_string());
            }
            "480p" => {
                args.push("-f".to_string());
                args.push("bestvideo[height<=480]+bestaudio/best[height<=480]".to_string());
            }
            _ => {
                args.push("-f".to_string());
                args.push("best".to_string());
            }
        }
    }

    // Playlist handling
    if !options.is_playlist {
        args.push("--no-playlist".to_string());
    }

    // Progress output
    args.push("--newline".to_string());
    args.push("--progress".to_string());

    // URL
    args.push(options.url.clone());

    // Execute yt-dlp
    let mut child = Command::new("yt-dlp")
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            let err_msg = if e.kind() == std::io::ErrorKind::NotFound {
                "yt-dlp not found. Please install yt-dlp first.\n\
                 Install with: pip install yt-dlp\n\
                 Or visit: https://github.com/yt-dlp/yt-dlp#installation"
                    .to_string()
            } else {
                format!("Failed to start yt-dlp: {}", e)
            };
            
            // Update state on error
            if let Ok(mut dl) = downloader.lock() {
                dl.state = DownloadState::Error(err_msg.clone());
            }
            err_msg
        })?;

    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    // Read output
    loop {
        tokio::select! {
            line = stdout_reader.next_line() => {
                match line {
                    Ok(Some(line)) => {
                        if let Ok(mut dl) = downloader.lock() {
                            dl.parse_progress(&line);
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        let err_msg = format!("Error reading output: {}", e);
                        if let Ok(mut dl) = downloader.lock() {
                            dl.state = DownloadState::Error(err_msg.clone());
                        }
                        return Err(format!("Error reading output: {}", e));
                    }
                }
            }
            line = stderr_reader.next_line() => {
                match line {
                    Ok(Some(line)) => {
                        if line.contains("ERROR") {
                            if let Ok(mut dl) = downloader.lock() {
                                dl.status_message = line.clone();
                            }
                        }
                    }
                    Ok(None) => {}
                    Err(_) => {}
                }
            }
        }
    }

    let status = child.wait().await.map_err(|e| format!("Failed to wait for process: {}", e))?;

    if status.success() {
        if let Ok(mut dl) = downloader.lock() {
            dl.state = DownloadState::Complete;
            dl.progress = 100.0;
            dl.status_message = "Download complete!".to_string();
        }
        Ok(())
    } else {
        let error_msg = format!("Download failed with exit code: {:?}", status.code());
        if let Ok(mut dl) = downloader.lock() {
            dl.state = DownloadState::Error(error_msg.clone());
        }
        Err(error_msg)
    }
}
