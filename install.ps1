# setproxy Windows Installer
# Usage: iwr -useb https://raw.githubusercontent.com/ChengHongxu/setproxy/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

# Detect architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "amd64" } else { "386" }
$asset = "setproxy-windows-$arch.exe"
$url = "https://github.com/ChengHongxu/setproxy/releases/latest/download/$asset"

Write-Host "🔍 Detecting system: Windows $arch" -ForegroundColor Cyan

# Determine install location
$installDir = "$env:LOCALAPPDATA\setproxy"
$binPath = "$installDir\setproxy.exe"

# Create directory
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

# Download binary
Write-Host "📥 Downloading $asset..." -ForegroundColor Cyan
try {
    Invoke-WebRequest -Uri $url -OutFile $binPath -UseBasicParsing
} catch {
    Write-Host "❌ Failed to download from $url" -ForegroundColor Red
    Write-Host "Please check your internet connection or try manual installation." -ForegroundColor Yellow
    exit 1
}

# Remove files installed by setproxy 0.2.x
Remove-Item "$installDir\_setproxy.exe", "$installDir\setproxy.ps1" -ErrorAction SilentlyContinue

# Add to PATH if not already there
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$installDir*") {
    Write-Host "➕ Adding to PATH..." -ForegroundColor Cyan
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$currentPath;$installDir",
        "User"
    )
    $env:Path = "$env:Path;$installDir"
}

# Running this installer is the explicit opt-in to editing $PROFILE
& $binPath init powershell

Write-Host ""
Write-Host "Then you can use:" -ForegroundColor Yellow
Write-Host "  setproxy 127.0.0.1 7890" -ForegroundColor Cyan
Write-Host "  setproxy status" -ForegroundColor Cyan
Write-Host "  setproxy clear" -ForegroundColor Cyan
