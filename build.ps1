# ytdwt build script for Windows

Write-Host "Building ytdwt in release mode..." -ForegroundColor Cyan

# Ensure we are in the right directory
$PSScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Definition
Set-Location $PSScriptRoot

# Run cargo build
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n[SUCCESS] Build successful!" -ForegroundColor Green
    
    $targetDir = "dist"
    if (-not (Test-Path $targetDir)) {
        New-Item -ItemType Directory -Path $targetDir | Out-Null
    }

    # Copy binary
    Copy-Item "target\release\ytdwt.exe" "$targetDir\ytdwt.exe" -Force
    
    Write-Host "`nExecutable location: $targetDir\ytdwt.exe"
    
    Write-Host "`nNext steps:" -ForegroundColor Yellow
    Write-Host "1. Download yt-dlp.exe from https://github.com/yt-dlp/yt-dlp/releases"
    Write-Host "2. Download ffmpeg.exe from https://www.gyan.dev/ffmpeg/builds/"
    Write-Host "3. Place both .exe files in the '$targetDir' folder with ytdwt.exe"
    Write-Host "4. Run ytdwt.exe!"
} else {
    Write-Host "`n[ERROR] Build failed!" -ForegroundColor Red
    exit 1
}
