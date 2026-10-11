#!/usr/bin/env bash
# setproxy wrapper - eval output from _setproxy binary
# This wrapper allows environment variable changes to affect the current shell
#
# IMPORTANT: This must be used as a function, not a script.
# Install it by sourcing in your shell RC file, or define as function:
#
#   setproxy() {
#       eval "$(_setproxy "$@")"
#   }

# When sourced as a function
setproxy() {
    local setproxy_bin
    setproxy_bin=$(command -v _setproxy 2>/dev/null)
    if [ -z "$setproxy_bin" ]; then
        echo "✗ _setproxy not found in PATH" >&2
        return 1
    fi

    # Execute _setproxy and eval its output
    eval "$("$setproxy_bin" "$@")"
}

# When executed directly (show usage)
if [ "${BASH_SOURCE[0]}" = "${0}" ]; then
    cat >&2 <<'EOF'
⚠️  setproxy wrapper should be used as a function, not executed directly.

Add this to your ~/.bashrc or ~/.zshrc:

    setproxy() {
        eval "$(_setproxy "$@")"
    }

Or source this file:

    source /path/to/setproxy-wrapper.sh

Then use:
    setproxy 127.0.0.1 7890
    setproxy status
    setproxy clear
EOF
    exit 1
fi
