mod cli;
mod proxy;
mod shell_init;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use proxy::{clear_proxy, detect_wsl_host, set_proxy, show_status, ProxyConfig};
use shell_init::{auto_configure_shell, is_shell_configured};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Auto-configure shell integration on first use (if not already configured)
    // This runs silently in the background - only shows message if it adds the config
    if !is_shell_configured() {
        // Only attempt auto-configure for non-shell-init commands
        // shell-init is meant to be used manually, don't auto-configure during it
        if !matches!(cli.command, Some(Commands::ShellInit)) {
            let _ = auto_configure_shell(); // Ignore errors, don't block actual command
        }
    }

    match cli.command {
        Some(Commands::Clear) => {
            clear_proxy()?;
        }
        Some(Commands::Status) => {
            show_status()?;
        }
        Some(Commands::ShellInit) => {
            // 输出 shell function 定义，供 eval 使用
            // 用法: eval "$(_setproxy shell-init)" 添加到 ~/.zshrc 或 ~/.bashrc
            println!(
                r#"setproxy() {{
    local _bin
    _bin=$(command -v _setproxy 2>/dev/null) || {{
        echo "✗ _setproxy not found, please install it first" >&2
        return 1
    }}
    if [ "$1" = "status" ] || [ "$1" = "s" ]; then
        "$_bin" "$@"
    else
        eval "$("$_bin" "$@")"
    fi
}}"#
            );
        }
        Some(Commands::Init) => {
            // 手动触发 shell 集成配置
            auto_configure_shell()?;
        }
        Some(Commands::Wsl { port }) => {
            // WSL2 自动检测模式
            match detect_wsl_host() {
                Ok(host) => {
                    let config = ProxyConfig::new(host, port);
                    set_proxy(&config)?;
                }
                Err(e) => {
                    eprintln!("{} {}", "✗".red().bold(), e);
                    std::process::exit(1);
                }
            }
        }
        None => {
            // 默认模式：设置代理
            if let (Some(host), Some(port)) = (cli.host, cli.port) {
                let config = ProxyConfig::new(host, port);
                set_proxy(&config)?;
            } else {
                // 没有参数，显示帮助
                eprintln!("{} 请提供代理地址和端口，或使用子命令", "✗".red().bold());
                eprintln!();
                eprintln!("示例:");
                eprintln!("  setproxy 192.168.1.1 8080    # 设置代理");
                eprintln!("  setproxy wsl                 # WSL2 自动检测");
                eprintln!("  setproxy clear               # 清除代理");
                eprintln!("  setproxy status              # 查看状态");
                eprintln!();
                eprintln!("使用 {} 查看完整帮助", "setproxy --help".cyan());
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
