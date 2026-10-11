use anyhow::Result;
use colored::*;
use std::env;

#[cfg(target_os = "linux")]
use anyhow::Context;
#[cfg(target_os = "linux")]
use std::process::Command;

/// Proxy configuration
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub host: String,
    pub port: u16,
}

impl ProxyConfig {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub fn http_url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    pub fn socks5_url(&self) -> String {
        format!("socks5://{}:{}", self.host, self.port)
    }
}

/// Detect WSL2 Windows host IP
pub fn detect_wsl_host() -> Result<String> {
    #[cfg(target_os = "linux")]
    {
        // Check if running in WSL environment
        if let Ok(wsl_distro) = env::var("WSL_DISTRO_NAME") {
            use std::io::IsTerminal;
            if std::io::stdout().is_terminal() {
                eprintln!("{} WSL: {}", "✓".green(), wsl_distro);
            }

            let output = Command::new("sh")
                .arg("-c")
                .arg("ip route | grep default | awk '{print $3}' | head -n1")
                .output()
                .context("Cannot get WSL host IP")?;

            if output.status.success() {
                let host = String::from_utf8_lossy(&output.stdout).trim().to_string();

                if !host.is_empty() {
                    return Ok(host);
                }
            }
        }
    }

    anyhow::bail!("Not in WSL2 environment or cannot get host IP")
}

/// Set proxy environment variables
pub fn set_proxy(config: &ProxyConfig) -> Result<()> {
    // Output shell commands for eval
    println!("export http_proxy=\"{}\"", config.http_url());
    println!("export https_proxy=\"{}\"", config.http_url());
    println!("export all_proxy=\"{}\"", config.socks5_url());
    println!("echo \"✓ 代理已启用: {}\"", config.http_url());
    Ok(())
}

/// Clear proxy environment variables
pub fn clear_proxy() -> Result<()> {
    // Output shell commands for eval
    println!("unset http_proxy");
    println!("unset https_proxy");
    println!("unset all_proxy");
    println!("echo \"✓ 代理已清除\"");
    Ok(())
}

/// Show current proxy status
pub fn show_status() -> Result<()> {
    eprintln!("{} 当前代理状态:", "🌐".bold());
    eprintln!();

    let vars = [
        "http_proxy",
        "https_proxy",
        "all_proxy",
        "HTTP_PROXY",
        "HTTPS_PROXY",
        "ALL_PROXY",
    ];
    let mut found = false;

    for var in &vars {
        if let Ok(value) = env::var(var) {
            if !value.is_empty() {
                eprintln!("  {} = {}", var.cyan(), value.green());
                found = true;
            }
        }
    }

    if !found {
        eprintln!("  {}", "未设置代理".yellow());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_config() {
        let config = ProxyConfig::new("192.168.1.1".to_string(), 8080);
        assert_eq!(config.http_url(), "http://192.168.1.1:8080");
        assert_eq!(config.socks5_url(), "socks5://192.168.1.1:8080");
    }
}
