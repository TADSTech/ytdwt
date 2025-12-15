# Warning!!!
## This is an hobyist project, I may not know how to write memory safe applications. I am not liable for your 12th gen I7 blowing up

# ytdwt - YouTube Downloader with Tools

Lightweight GUI for downloading YouTube videos/audio via `yt-dlp`.

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## Features
- Video Downloads
- Audio Extraction
- Playlist Support
- Progress Tracking
- Settings Persistence

## Prerequisites

### Required
- **yt-dlp**
  ```bash
  pip install yt-dlp
  ```

### For building from source
- **Rust** (1.70+)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

## Installation

### Build from Source

1.  **Clone**
    ```bash
    git clone https://github.com/tadstech/ytdwt.git
    cd ytdwt
    ```

2.  **Build**
    ```bash
    chmod +x build.sh
    ./build.sh
    ```

3.  **Run**
    ```bash
    ./target/release/ytdwt
    ```

## Usage
1.  Enter YouTube URL.
2.  Select format (Video/Audio) & quality.
3.  Set output directory.
4.  Check "Download entire playlist" for playlists.
5.  Click Download.

## Building for Distribution

### Create Optimized Binary
```bash
cargo build --release --target x86_64-unknown-linux-gnu
strip target/x86_64-unknown-linux-gnu/release/ytdwt
```

### Create .deb Package
```bash
cargo install cargo-deb
cargo deb
```

## Configuration

Linux: `~/.config/ytdwt/config.json`
```json
{
  "default_output_dir": "/home/user/Downloads",
  "default_quality": "1080p",
  "default_format": "Video"
}
```

## Troubleshooting

### "yt-dlp not found"
Ensure `yt-dlp` is installed and in your PATH.
```bash
which yt-dlp
```

### Build errors
Update Rust: `rustup update`.
Install system dependencies (Ubuntu/Debian):
```bash
sudo apt install build-essential pkg-config libssl-dev
```

## Technology Stack
Rust, egui, eframe, tokio, yt-dlp.

## License
MIT License.

## Contributing
Contributions welcome via Pull Request.

## Acknowledgments
yt-dlp, egui.
