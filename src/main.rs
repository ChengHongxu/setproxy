mod cli;
mod proxy;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use proxy::{clear_proxy, detect_wsl_host, set_proxy, show_status, ProxyConfig};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Clear) => {
            clear_proxy()?;
        }
        Some(Commands::Status) => {
            show_status()?;
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
