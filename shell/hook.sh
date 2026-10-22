# setproxy shell integration for bash and zsh.
# `setproxy init` adds this line to ~/.zshrc or ~/.bashrc:
#   eval "$(setproxy hook zsh)"
#
# A child process cannot change its parent's environment, so the setproxy
# executable prints shell code and this function evals it in the current shell.

setproxy() {
    # Read-only commands print to the terminal; nothing to eval
    case "${1:-}" in
        status|s|init|hook|help)
            command setproxy "$@"
            return
            ;;
    esac
    local __setproxy_arg
    for __setproxy_arg in "$@"; do
        case "$__setproxy_arg" in
            -h|--help|-V|--version)
                command setproxy "$@"
                return
                ;;
        esac
    done

    local __setproxy_code
    __setproxy_code=$(command setproxy --syntax posix "$@") || return
    eval "$__setproxy_code"
}
