use crate::cli::Syntax;
use anyhow::Result;
use colored::*;
use std::env;

#[cfg(target_os = "linux")]
use anyhow::Context;
#[cfg(target_os = "linux")]
use std::process::Command;

/// Variables managed by setproxy
const PROXY_VARS: [&str; 3] = ["http_proxy", "https_proxy", "all_proxy"];

/// Proxy configuration
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub host: String,
    pub port: u16,
}

impl ProxyConfig {
    /// The host ends up inside shell code that gets eval'd, so only accept
    /// characters that can appear in a hostname or IP address.
    pub fn new(host: String, port: u16) -> Result<Self> {
        let valid = !host.is_empty()
            && host.chars().all(|c| {
                c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | ':' | '[' | ']')
            });
        if !valid {
            anyhow::bail!("无效的代理地址: {}", host);
        }
        Ok(Self { host, port })
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

/// Print code that sets the variable, in the given syntax
fn set_var(syntax: Syntax, name: &str, value: &str) -> String {
    // Values are validated in ProxyConfig::new and contain no quotes
    match syntax {
        Syntax::Posix => format!("export {}='{}'", name, value),
        Syntax::Powershell => format!("$env:{} = '{}'", name, value),
    }
}

/// Print code that removes the variable, in the given syntax
fn unset_var(syntax: Syntax, name: &str) -> String {
    match syntax {
        Syntax::Posix => format!("unset {}", name),
        Syntax::Powershell => format!("Remove-Item Env:{} -ErrorAction SilentlyContinue", name),
    }
}

/// Set proxy environment variables
pub fn set_proxy(config: &ProxyConfig, syntax: Syntax) -> Result<()> {
    // stdout is code for the shell function to evaluate; messages go to stderr
    let values = [config.http_url(), config.http_url(), config.socks5_url()];
    for (name, value) in PROXY_VARS.iter().zip(&values) {
        println!("{}", set_var(syntax, name, value));
    }
    eprintln!("{} 代理已启用: {}", "✓".green().bold(), config.http_url());
    Ok(())
}

/// Clear proxy environment variables
pub fn clear_proxy(syntax: Syntax) -> Result<()> {
    for name in PROXY_VARS {
        println!("{}", unset_var(syntax, name));
    }
    eprintln!("{} 代理已清除", "✓".green().bold());
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
        let config = ProxyConfig::new("192.168.1.1".to_string(), 8080).unwrap();
        assert_eq!(config.http_url(), "http://192.168.1.1:8080");
        assert_eq!(config.socks5_url(), "socks5://192.168.1.1:8080");
    }

    #[test]
    fn test_rejects_shell_metacharacters() {
        for host in ["", "a;rm -rf ~", "$(id)", "a'b", "a b", "`id`"] {
            assert!(
                ProxyConfig::new(host.to_string(), 8080).is_err(),
                "{host:?}"
            );
        }
        assert!(ProxyConfig::new("[::1]".to_string(), 8080).is_ok());
        assert!(ProxyConfig::new("proxy.local".to_string(), 8080).is_ok());
    }

    #[test]
    fn test_syntax() {
        assert_eq!(
            set_var(Syntax::Posix, "http_proxy", "x"),
            "export http_proxy='x'"
        );
        assert_eq!(
            set_var(Syntax::Powershell, "http_proxy", "x"),
            "$env:http_proxy = 'x'"
        );
        assert_eq!(unset_var(Syntax::Posix, "all_proxy"), "unset all_proxy");
    }
}
