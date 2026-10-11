# setproxy - set proxy environment variables
# Compatible with bash and zsh.
# Source this file in your .zshrc / .bashrc:
#   source /path/to/shell/setproxy.sh

setproxy() {
    local binary
    binary=$(command -v _setproxy 2>/dev/null) || {
        echo "✗ _setproxy not found, please install it first" >&2
        return 1
    }

    # Special case: status command - don't eval, just pass through
    if [ "$1" = "status" ] || [ "$1" = "s" ]; then
        "$binary" "$@"
        return $?
    fi

    # Execute _setproxy and eval its output to modify current shell environment
    eval "$("$binary" "$@")"
}
