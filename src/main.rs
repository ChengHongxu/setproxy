mod cli;
mod proxy;
mod shell;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, Syntax};
use colored::*;
use proxy::{clear_proxy, detect_wsl_host, set_proxy, show_status, ProxyConfig};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Clear) => {
            clear_proxy(require_shell_function(cli.syntax))?;
        }
        Some(Commands::Status) => {
            show_status()?;
        }
        Some(Commands::Init { shell }) => {
            shell::install(shell::resolve(shell)?)?;
        }
        Some(Commands::Hook { shell }) => {
            // 启动文件中执行: eval "$(setproxy hook zsh)"
            print!("{}", shell::hook_script(shell));
        }
        Some(Commands::Wsl { port }) => {
            let syntax = require_shell_function(cli.syntax);
            // WSL2 自动检测模式
            match detect_wsl_host() {
                Ok(host) => {
                    let config = ProxyConfig::new(host, port)?;
                    set_proxy(&config, syntax)?;
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
                let syntax = require_shell_function(cli.syntax);
                let config = ProxyConfig::new(host, port)?;
                set_proxy(&config, syntax)?;
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

/// Only the shell function (which passes --syntax) can apply the printed
/// code to the current shell; run directly, explain how to set it up.
fn require_shell_function(syntax: Option<Syntax>) -> Syntax {
    syntax.unwrap_or_else(|| {
        shell::print_not_loaded_hint();
        std::process::exit(1)
    })
}
