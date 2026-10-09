# setproxy - set proxy environment variables (PowerShell)
# Load in your PowerShell profile:
#   . ~/path/to/shell/setproxy.ps1

function setproxy {
    $binary = Get-Command _setproxy -ErrorAction SilentlyContinue
    if (-not $binary) {
        Write-Error "✗ _setproxy not found, please install it first"
        return
    }

    $output = & $binary.Source @args 2>&1
    if ($LASTEXITCODE -ne 0) { Write-Error $output; return }

    if ($output -match "^SET (\S+) (\S+)") {
        $env:http_proxy  = $Matches[1]
        $env:https_proxy = $Matches[1]
        $env:all_proxy   = $Matches[2]
        Write-Host "✓ 代理已启用: $($Matches[1])" -ForegroundColor Green
    } elseif ($output -eq "CLEAR") {
        Remove-Item Env:\http_proxy  -ErrorAction SilentlyContinue
        Remove-Item Env:\https_proxy -ErrorAction SilentlyContinue
        Remove-Item Env:\all_proxy   -ErrorAction SilentlyContinue
        Write-Host "✓ 代理已清除" -ForegroundColor Green
    } else {
        Write-Output $output
    }
}
