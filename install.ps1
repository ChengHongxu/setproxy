# setproxy Windows Installer
# Usage: iwr -useb https://raw.githubusercontent.com/<your-username>/setproxy/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

# Detect architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "amd64" } else { "386" }
$asset = "_setproxy-windows-$arch.exe"
$url = "https://github.com/<your-username>/setproxy/releases/latest/download/$asset"

Write-Host "🔍 Detecting system: Windows $arch" -ForegroundColor Cyan

# Determine install location
$installDir = "$env:LOCALAPPDATA\setproxy"
$binPath = "$installDir\_setproxy.exe"

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

# Download shell function
$shellFuncUrl = "https://raw.githubusercontent.com/<your-username>/setproxy/main/shell/setproxy.ps1"
$shellFuncPath = "$installDir\setproxy.ps1"

Write-Host "📥 Downloading PowerShell function..." -ForegroundColor Cyan
try {
    Invoke-WebRequest -Uri $shellFuncUrl -OutFile $shellFuncPath -UseBasicParsing
} catch {
    Write-Host "❌ Failed to download shell function" -ForegroundColor Red
    exit 1
}

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

# Setup PowerShell profile
$profileDir = Split-Path $PROFILE -Parent
if (-not (Test-Path $profileDir)) {
    New-Item -ItemType Directory -Path $profileDir -Force | Out-Null
}

$sourceCmd = ". `"$shellFuncPath`""
if (Test-Path $PROFILE) {
    $profileContent = Get-Content $PROFILE -Raw
    if ($profileContent -notlike "*setproxy.ps1*") {
        Write-Host "➕ Adding to PowerShell profile..." -ForegroundColor Cyan
        Add-Content -Path $PROFILE -Value "`n# setproxy`n$sourceCmd"
    }
} else {
    Write-Host "➕ Creating PowerShell profile..." -ForegroundColor Cyan
    "# setproxy`n$sourceCmd" | Out-File -FilePath $PROFILE -Encoding UTF8
}

Write-Host ""
Write-Host "✅ Installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Please restart your PowerShell or run:" -ForegroundColor Yellow
Write-Host "  . `$PROFILE" -ForegroundColor Cyan
Write-Host ""
Write-Host "Then you can use:" -ForegroundColor Yellow
Write-Host "  setproxy 127.0.0.1 7890" -ForegroundColor Cyan
Write-Host "  setproxy status" -ForegroundColor Cyan
Write-Host "  setproxy clear" -ForegroundColor Cyan
