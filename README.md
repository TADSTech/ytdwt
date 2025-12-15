# ytdwt - YouTube Downloader with Tools

A lightweight, fast GUI application for downloading YouTube videos and audio using yt-dlp.

![ytdwt](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## Features

- 🎬 **Video Downloads** - Download videos in various qualities (Best, 1080p, 720p, 480p)
- 🎵 **Audio Extraction** - Extract audio and convert to MP3
- 📋 **Playlist Support** - Download entire playlists with one click
- 📊 **Progress Tracking** - Real-time download progress and status
- 💾 **Settings Persistence** - Save your preferred output directory and quality
- 🪶 **Lightweight** - Small binary size (~5-10MB) with minimal dependencies
- ⚡ **Fast** - Built with Rust and egui for maximum performance

## Prerequisites

### Required
- **yt-dlp** - The download engine
  ```bash
  # Install via pip
  pip install yt-dlp
  
  # Or via package manager (Ubuntu/Debian)
  sudo apt install yt-dlp
  
  # Or download binary from https://github.com/yt-dlp/yt-dlp#installation
  ```

### Optional (for building from source)
- **Rust** - Version 1.70 or higher
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

## Installation

### Option 1: Download Pre-built Binary (Coming Soon)
Download the latest release from the [Releases](https://github.com/tadstech/ytdwt/releases) page.

### Option 2: Build from Source

1. **Clone the repository**
   ```bash
   git clone https://github.com/tadstech/ytdwt.git
   cd ytdwt
   ```

2. **Build the project**
   ```bash
   chmod +x build.sh
   ./build.sh
   ```

3. **Install system-wide (optional)**
   ```bash
   sudo cp target/release/ytdwt /usr/local/bin/
   ```

4. **Run the application**
   ```bash
   ytdwt
   # Or if not installed system-wide:
   ./target/release/ytdwt
   ```

## Usage

1. **Enter URL** - Paste a YouTube video or playlist URL
2. **Select Format** - Choose between Video or Audio Only (MP3)
3. **Choose Quality** - Select video quality (for video downloads)
4. **Set Output Directory** - Choose where to save downloads
5. **Playlist Option** - Check if downloading an entire playlist
6. **Download** - Click the download button and watch the progress!

### Tips
- Use "Save Settings" to remember your preferred output directory and quality
- The app supports both full URLs and video IDs
- For playlists, enable the "Download entire playlist" checkbox

## Building for Distribution

### Create Optimized Binary
```bash
cargo build --release --target x86_64-unknown-linux-gnu
strip target/x86_64-unknown-linux-gnu/release/ytdwt
```

### Create .deb Package (Ubuntu/Debian)
```bash
# Install cargo-deb
cargo install cargo-deb

# Build .deb package
cargo deb
```

The .deb package will be in `target/debian/`

### Create AppImage
```bash
# Install linuxdeploy
wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
chmod +x linuxdeploy-x86_64.AppImage

# Create AppImage
./linuxdeploy-x86_64.AppImage --appdir AppDir --executable target/release/ytdwt --desktop-file ytdwt.desktop --output appimage
```

## Configuration

Configuration is stored in:
- Linux: `~/.config/ytdwt/config.json`

Example config:
```json
{
  "default_output_dir": "/home/user/Downloads",
  "default_quality": "1080p",
  "default_format": "Video"
}
```

## Troubleshooting

### "yt-dlp not found" error
Make sure yt-dlp is installed and in your PATH:
```bash
which yt-dlp
yt-dlp --version
```

### Download fails
- Check your internet connection
- Verify the URL is correct
- Try updating yt-dlp: `pip install -U yt-dlp`
- Some videos may be region-restricted or require authentication

### Build errors
- Ensure you have the latest Rust version: `rustup update`
- Install required system dependencies:
  ```bash
  # Ubuntu/Debian
  sudo apt install build-essential pkg-config libssl-dev
  ```

## Technology Stack

- **Rust** - Systems programming language
- **egui** - Immediate mode GUI library
- **eframe** - Framework for native egui apps
- **tokio** - Async runtime for process execution
- **yt-dlp** - YouTube download engine

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - The powerful download engine
- [egui](https://github.com/emilk/egui) - The excellent immediate mode GUI library
