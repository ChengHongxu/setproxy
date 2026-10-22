# setproxy shell integration for PowerShell.
# `setproxy init` adds this line to $PROFILE:
#   Invoke-Expression (& setproxy hook powershell | Out-String)
#
# The setproxy executable prints PowerShell code and this function evaluates
# it in the current session.

function setproxy {
    # The setproxy executable, not this function
    $exe = Get-Command -Name setproxy -CommandType Application -ErrorAction Stop | Select-Object -First 1

    # Read-only commands print to the terminal; nothing to evaluate
    $readOnly = ($args.Count -gt 0 -and $args[0] -in @('status', 's', 'init', 'hook', 'help')) -or
        ($args | Where-Object { $_ -in @('-h', '--help', '-V', '--version') })
    if ($readOnly) {
        & $exe @args
        return
    }

    $code = & $exe --syntax powershell @args
    if ($LASTEXITCODE -ne 0 -or -not $code) { return }
    Invoke-Expression ($code | Out-String)
}
