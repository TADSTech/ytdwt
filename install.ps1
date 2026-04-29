# ytdwt Installation Script for Windows
# This script copies the application to Program Files and creates shortcuts.

$PSScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Definition
Set-Location $PSScriptRoot

# Check for Administrator privileges
$currentPrincipal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
$isAdministrator = $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdministrator) {
    Write-Host "Requesting Administrator privileges..." -ForegroundColor Yellow
    Start-Process powershell.exe -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
    exit
}

Write-Host "Installing ytdwt..." -ForegroundColor Cyan

$appName = "ytdwt"
$installDir = Join-Path $env:ProgramFiles $appName
$sourceDir = Join-Path $PSScriptRoot "dist"
$iconSource = Join-Path $PSScriptRoot "app_icon.ico"

# 1. Create Installation Directory
if (-not (Test-Path $installDir)) {
    Write-Host "Creating directory: $installDir"
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

# 2. Copy Files
if (Test-Path $sourceDir) {
    Write-Host "Copying application files..."
    Copy-Item -Path "$sourceDir\*" -Destination $installDir -Recurse -Force
} else {
    Write-Host "[ERROR] 'dist' folder not found. Please run build.ps1 first." -ForegroundColor Red
    exit 1
}

# Copy icon to install dir for the shortcut
if (Test-Path $iconSource) {
    Copy-Item -Path $iconSource -Destination $installDir -Force
}

$exePath = Join-Path $installDir "ytdwt.exe"
$iconPath = Join-Path $installDir "app_icon.ico"

# Function to create a shortcut
function Create-Shortcut {
    param (
        [string]$SourcePath,
        [string]$ShortcutPath,
        [string]$IconPath
    )
    $WshShell = New-Object -ComObject WScript.Shell
    $Shortcut = $WshShell.CreateShortcut($ShortcutPath)
    $Shortcut.TargetPath = $SourcePath
    $Shortcut.WorkingDirectory = Split-Path $SourcePath
    if (Test-Path $IconPath) {
        $Shortcut.IconLocation = $IconPath
    }
    $Shortcut.Save()
}

# 3. Create Desktop Shortcut
$desktopPath = [System.IO.Path]::Combine([Environment]::GetFolderPath("CommonDesktopDirectory"), "$appName.lnk")
Write-Host "Creating Desktop shortcut..."
Create-Shortcut -SourcePath $exePath -ShortcutPath $desktopPath -IconPath $iconPath

# 4. Create Start Menu Shortcut
$startMenuPathDir = [System.IO.Path]::Combine([Environment]::GetFolderPath("CommonPrograms"), $appName)
if (-not (Test-Path $startMenuPathDir)) {
    New-Item -ItemType Directory -Path $startMenuPathDir -Force | Out-Null
}
$startMenuPath = Join-Path $startMenuPathDir "$appName.lnk"
Write-Host "Creating Start Menu shortcut..."
Create-Shortcut -SourcePath $exePath -ShortcutPath $startMenuPath -IconPath $iconPath

Write-Host "`n[SUCCESS] ytdwt has been installed successfully!" -ForegroundColor Green
Write-Host "You can now find it on your Desktop and in the Start Menu."
Pause
