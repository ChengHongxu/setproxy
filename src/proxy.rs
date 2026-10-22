use crate::cli::Syntax;
use anyhow::Result;
use colored::*;
use std::env;

#[cfg(target_os = "linux")]
use anyhow::Context;
#[cfg(target_os = "linux")]
use std::process::Command;

/// Variables managed by setproxy. Tools disagree on which case they read
/// (curl ignores HTTP_PROXY, many Go and Java tools read only upper case),
/// and WSL's autoProxy sets both, so both are set and cleared.
const PROXY_VARS: [&str; 6] = [
    "http_proxy",
    "https_proxy",
    "all_proxy",
    "HTTP_PROXY",
    "HTTPS_PROXY",
    "ALL_PROXY",
];

/// Environment variable names are case-insensitive on Windows, where the
/// upper-case names would refer to the same variables again
fn proxy_vars() -> &'static [&'static str] {
    if cfg!(windows) {
        &PROXY_VARS[..3]
    } else {
        &PROXY_VARS
    }
}

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

    /// all_proxy gets the SOCKS URL, the others the HTTP URL
    fn url_for(&self, var: &str) -> String {
        if var.eq_ignore_ascii_case("all_proxy") {
            self.socks5_url()
        } else {
            self.http_url()
        }
    }
}

/// Detect WSL2 Windows host IP
pub fn detect_wsl_host() -> Result<String> {
    #[cfg(target_os = "linux")]
    {
        // Check if running in WSL environment
        if let Ok(wsl_distro) = env::var("WSL_DISTRO_NAME") {
            let mirrored = wsl_networking_mode().as_deref() == Some("mirrored");

            use std::io::IsTerminal;
            if std::io::stdout().is_terminal() {
                let mode = if mirrored { "mirrored" } else { "NAT" };
                eprintln!("{} WSL: {} ({})", "✓".green(), wsl_distro, mode);
            }

            // Mirrored networking shares localhost with Windows
            if mirrored {
                return Ok("127.0.0.1".to_string());
            }

            // NAT networking: the Windows host is the default gateway
            let output = Command::new("ip")
                .args(["route", "show", "default"])
                .output()
                .context("Cannot get WSL host IP")?;

            if output.status.success() {
                let routes = String::from_utf8_lossy(&output.stdout);
                if let Some(host) = routes.split_whitespace().skip_while(|w| *w != "via").nth(1) {
                    return Ok(host.to_string());
                }
            }
        }
    }

    anyhow::bail!("Not in WSL2 environment or cannot get host IP")
}

/// "nat" or "mirrored", from `wslinfo` (WSL 2.0+)
#[cfg(target_os = "linux")]
fn wsl_networking_mode() -> Option<String> {
    let output = Command::new("wslinfo")
        .arg("--networking-mode")
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
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
    for name in proxy_vars() {
        println!("{}", set_var(syntax, name, &config.url_for(name)));
    }
    eprintln!("{} 代理已启用: {}", "✓".green().bold(), config.http_url());
    Ok(())
}

/// Clear proxy environment variables
pub fn clear_proxy(syntax: Syntax) -> Result<()> {
    for name in proxy_vars() {
        println!("{}", unset_var(syntax, name));
    }
    eprintln!("{} 代理已清除", "✓".green().bold());
    Ok(())
}

/// Show current proxy status
pub fn show_status() -> Result<()> {
    eprintln!("{} 当前代理状态:", "🌐".bold());
    eprintln!();

    let mut found = false;

    for var in proxy_vars() {
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
        assert_eq!(config.url_for("ALL_PROXY"), "socks5://192.168.1.1:8080");
        assert_eq!(config.url_for("HTTPS_PROXY"), "http://192.168.1.1:8080");
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
