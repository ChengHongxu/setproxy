# setproxy wrapper for PowerShell
# Eval output from _setproxy.exe binary

$setproxyBin = Get-Command _setproxy -ErrorAction SilentlyContinue
if (-not $setproxyBin) {
    Write-Error "✗ _setproxy.exe not found in PATH"
    exit 1
}

# Execute _setproxy and eval its output (PowerShell commands)
$output = & _setproxy @args
if ($LASTEXITCODE -eq 0) {
    # Parse and execute commands
    $output | ForEach-Object {
        if ($_ -match '^export (.+?)="(.+)"$') {
            Set-Item -Path "env:$($matches[1])" -Value $matches[2]
        } elseif ($_ -match '^unset (.+)$') {
            Remove-Item -Path "env:$($matches[1])" -ErrorAction SilentlyContinue
        } elseif ($_ -match '^echo "(.+)"$') {
            Write-Host $matches[1]
        }
    }
} else {
    Write-Error $output
    exit $LASTEXITCODE
}
