# setproxy - set proxy environment variables
# Source this file in your .bashrc:
#   source /path/to/setproxy.bash

setproxy() {
    local binary
    binary=$(command -v _setproxy 2>/dev/null) || {
        echo "✗ _setproxy 未找到，请确保已安装" >&2; return 1
    }

    # status/s: pass through directly
    if [[ "$1" == "status" || "$1" == "s" ]]; then
        "$binary" "$@"
        return $?
    fi

    # Other commands: capture stdout, stderr goes directly to terminal
    local output
    output=$("$binary" "$@")
    local exit_code=$?

    [[ $exit_code -ne 0 ]] && { echo "$output" >&2; return $exit_code; }

    local action
    action="${output%% *}"

    case "$action" in
        SET)
            local http_url socks5_url
            http_url=$(echo "$output" | awk '{print $2}')
            socks5_url=$(echo "$output" | awk '{print $3}')
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
