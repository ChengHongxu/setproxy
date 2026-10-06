use anyhow::{Context, Result};
use colored::*;
use std::env;
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
        // 检查是否在 WSL 环境中
        if let Ok(wsl_distro) = env::var("WSL_DISTRO_NAME") {
            // 只在交互模式下输出提示信息
            use std::io::IsTerminal;
            if std::io::stdout().is_terminal() {
                eprintln!("{} WSL 环境: {}", "✓".green(), wsl_distro);
            }

            // 从 /etc/resolv.conf 获取 nameserver (WSL2 的宿主机 IP)
            let output = Command::new("sh")
                .arg("-c")
                .arg("ip route | grep default | awk '{print $3}' | head -n1")
                .output()
                .context("无法执行命令获取 WSL 宿主机 IP")?;

            if output.status.success() {
                let host = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_string();

                if !host.is_empty() {
                    return Ok(host);
                }
            }
        }
    }

    anyhow::bail!("未检测到 WSL2 环境或无法获取宿主机 IP")
}

/// Set proxy environment variables
pub fn set_proxy(config: &ProxyConfig) -> Result<()> {
    let http_url = config.http_url();
    let socks5_url = config.socks5_url();

    // 生成 shell 脚本用于设置环境变量
    let script = generate_set_script(&http_url, &socks5_url);

    // 检测是否在管道中运行（stdout 被重定向）
    use std::io::IsTerminal;
    let is_piped = !std::io::stdout().is_terminal();

    if is_piped {
        // 在管道中，只输出脚本（用于 eval）
        print!("{}", script);
    } else {
        // 交互模式，显示详细信息
        println!("{} 设置代理...", "→".blue().bold());
        println!("  HTTP/HTTPS: {}", http_url.cyan());
        println!("  SOCKS5:     {}", socks5_url.cyan());
        println!();
        println!("{}", "请运行以下命令以在当前 shell 中生效:".yellow().bold());
        println!();

        #[cfg(target_os = "windows")]
        {
            println!("  {}", "# PowerShell".bright_black());
            println!("  {}", format!("setproxy {} {} | Invoke-Expression", config.host, config.port).green());
        }

        #[cfg(not(target_os = "windows"))]
        {
            println!("  {}", "# Bash/Zsh".bright_black());
            println!("  {}", format!("eval \"$(setproxy {} {})\"", config.host, config.port).green());
        }

        println!();
        println!("{}", "或者复制以下命令到终端执行:".yellow());
        println!();
        println!("{}", script.trim());
    }

    Ok(())
}

/// Clear proxy environment variables
pub fn clear_proxy() -> Result<()> {
    let script = generate_clear_script();

    // 检测是否在管道中运行（stdout 被重定向）
    use std::io::IsTerminal;
    let is_piped = !std::io::stdout().is_terminal();

    if is_piped {
        // 在管道中，只输出脚本（用于 eval）
        print!("{}", script);
    } else {
        // 交互模式，显示详细信息
        println!("{} 清除代理设置...", "→".blue().bold());
        println!();
        println!("{}", "请运行以下命令以在当前 shell 中生效:".yellow().bold());
        println!();

        #[cfg(target_os = "windows")]
        {
            println!("  {}", "# PowerShell".bright_black());
            println!("  {}", "setproxy clear | Invoke-Expression".green());
        }

        #[cfg(not(target_os = "windows"))]
        {
            println!("  {}", "# Bash/Zsh".bright_black());
            println!("  {}", "eval \"$(setproxy clear)\"".green());
        }

        println!();
        println!("{}", "或者复制以下命令到终端执行:".yellow());
        println!();
        println!("{}", script.trim());
    }

    Ok(())
}

/// Show current proxy status
pub fn show_status() -> Result<()> {
    println!("{} 当前代理状态:", "🌐".bold());
    println!();

    let vars = ["http_proxy", "https_proxy", "all_proxy", "HTTP_PROXY", "HTTPS_PROXY", "ALL_PROXY"];
    let mut found = false;

    for var in &vars {
        if let Ok(value) = env::var(var) {
            if !value.is_empty() {
                println!("  {} = {}", var.cyan(), value.green());
                found = true;
            }
        }
    }

    if !found {
        println!("  {}", "未设置代理".yellow());
    }

    Ok(())
}

/// Generate shell script to set proxy
fn generate_set_script(http_url: &str, socks5_url: &str) -> String {
    #[cfg(target_os = "windows")]
    {
        format!(
            r#"$env:http_proxy = "{}"
$env:https_proxy = "{}"
$env:all_proxy = "{}"
Write-Host "✓ 代理已启用: {}" -ForegroundColor Green"#,
            http_url, http_url, socks5_url, http_url
        )
    }

    #[cfg(not(target_os = "windows"))]
    {
        format!(
            r#"export http_proxy="{}"
export https_proxy="{}"
export all_proxy="{}"
echo "✓ 代理已启用: {}""#,
            http_url, http_url, socks5_url, http_url
        )
    }
}

/// Generate shell script to clear proxy
fn generate_clear_script() -> String {
    #[cfg(target_os = "windows")]
    {
        r#"Remove-Item Env:\http_proxy -ErrorAction SilentlyContinue
Remove-Item Env:\https_proxy -ErrorAction SilentlyContinue
Remove-Item Env:\all_proxy -ErrorAction SilentlyContinue
Write-Host "✓ 代理已清除" -ForegroundColor Green"#
            .to_string()
    }

    #[cfg(not(target_os = "windows"))]
    {
        r#"unset http_proxy
unset https_proxy
unset all_proxy
echo "✓ 代理已清除""#
            .to_string()
    }
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

    #[test]
    fn test_generate_scripts() {
        let http_url = "http://192.168.1.1:8080";
        let socks5_url = "socks5://192.168.1.1:8080";

        let set_script = generate_set_script(http_url, socks5_url);
        assert!(set_script.contains(http_url));

        let clear_script = generate_clear_script();
        assert!(clear_script.contains("unset") || clear_script.contains("Remove-Item"));
    }
}
