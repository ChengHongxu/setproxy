# setproxy - set proxy environment variables
# Compatible with bash and zsh.
# Source this file in your .zshrc / .bashrc:
#   source /path/to/shell/setproxy.sh

setproxy() {
    local binary
    binary=$(command -v _setproxy 2>/dev/null) || {
        echo "✗ _setproxy not found, please install it first" >&2; return 1
    }

    # status/s: pass through directly
    if [ "$1" = "status" ] || [ "$1" = "s" ]; then
        "$binary" "$@"
        return $?
    fi

    # Other commands: capture stdout, stderr goes directly to terminal
    local output exit_code
    output=$("$binary" "$@")
    exit_code=$?

    [ $exit_code -ne 0 ] && { echo "$output" >&2; return $exit_code; }

    local action http_url socks5_url
    action="${output%% *}"
    http_url=$(echo "$output" | awk '{print $2}')
    socks5_url=$(echo "$output" | awk '{print $3}')

    case "$action" in
        SET)
            export http_proxy="$http_url"
            export https_proxy="$http_url"
            export all_proxy="$socks5_url"
            echo "✓ 代理已启用: $http_url"
            ;;
        CLEAR)
            unset http_proxy https_proxy all_proxy
            echo "✓ 代理已清除"
            ;;
        *)
            echo "$output"
            ;;
    esac
}
