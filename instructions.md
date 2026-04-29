# ytdwt Windows Build Instructions

This guide will help you build and run the `ytdwt` YouTube Downloader on Windows.

## 0. Install C++ Build Tools (Required for Rust)

Rust on Windows requires the MSVC linker (`link.exe`). If you see an error about `link.exe` not found, follow these steps:

1. Download the [Visual Studio Installer](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
2. Run the installer and select **"Desktop development with C++"**.
3. Ensure the following are checked in the details pane:
   - MSVC v143 - VS 2022 C++ x64/x86 build tools
   - Windows 11 SDK (or Windows 10 SDK)
4. Click **Install** and restart your computer if prompted.

## 1. Prerequisites

- **Rust**: I've already repaired your local toolchain (`stable-x86_64-pc-windows-msvc`).
- **External Tools**: This app requires `yt-dlp` and `ffmpeg`.
  - [Download yt-dlp.exe](https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe)
  - [Download ffmpeg.exe](https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip) (Extract `ffmpeg.exe` and `ffprobe.exe` from the `bin` folder)

## 2. Building the Application

I've provided a PowerShell script to automate the build and packaging process.

1. Open PowerShell in the project root.
2. Run the build script:
   ```powershell
   .\build.ps1
   ```
   *Note: If you get a script execution policy error, run `Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope Process` first.*

This script will:
- Compile the app in `--release` mode (optimizing for size and speed).
- Create a `dist` folder.
- Copy the generated `ytdwt.exe` to the `dist` folder.

## 3. Final Setup

To make the app functional, follow these steps:

1. Go to the newly created `dist` folder.
2. Place the `yt-dlp.exe`, `ffmpeg.exe`, and `ffprobe.exe` you downloaded earlier into this `dist` folder.
3. Your folder should look like this:
   ```text
   dist/
   ├── ytdwt.exe
   ├── yt-dlp.exe
   ├── ffmpeg.exe
   └── ffprobe.exe
   ```

## 4. Running the App

Double-click `ytdwt.exe` in the `dist` folder. 
- The app will now run **without** a command prompt window opening.
- It will have the new professional red icon.
- It will automatically use the local `yt-dlp.exe` for downloads.

---

### Troubleshooting
- **Crates.io Index Update**: If `cargo` hangs while "Updating crates.io index", it's usually just a slow connection. Let it run; it will eventually finish.
- **Missing DLLs**: If the app fails to start, ensure you have the [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe) installed.
