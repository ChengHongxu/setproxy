use crate::cli::Shell;
use anyhow::{Context, Result};
use colored::*;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;

const POSIX_HOOK: &str = include_str!("../shell/hook.sh");
const POWERSHELL_HOOK: &str = include_str!("../shell/hook.ps1");

/// Comment written above the hook line
const COMMENT: &str = "# setproxy";

/// Shell function printed by `setproxy hook <shell>`
pub fn hook_script(shell: Shell) -> &'static str {
    match shell {
        Shell::Zsh | Shell::Bash => POSIX_HOOK,
        Shell::Powershell => POWERSHELL_HOOK,
    }
}

/// Line that `setproxy init` adds to the startup file
fn hook_line(shell: Shell) -> &'static str {
    match shell {
        Shell::Zsh => r#"eval "$(setproxy hook zsh)""#,
        Shell::Bash => r#"eval "$(setproxy hook bash)""#,
        Shell::Powershell => "Invoke-Expression (& setproxy hook powershell | Out-String)",
    }
}

pub fn resolve(shell: Option<Shell>) -> Result<Shell> {
    shell
        .or_else(detect)
        .context("无法识别当前 shell，请指定: setproxy init <zsh|bash|pwsh>")
}

/// `setproxy init`: add the hook line to the shell's startup file(s)
pub fn install(shell: Shell) -> Result<()> {
    let files = startup_files(shell)?;
    for file in &files {
        install_into(file, hook_line(shell))?;
    }
    eprintln!(
        "{} 重开终端，或运行: {}",
        "→".cyan(),
        reload_command(shell, &files[0]).yellow()
    );
    Ok(())
}

/// Called without the shell function loaded
pub fn print_not_loaded_hint() {
    let configured = detect()
        .and_then(|shell| Some((shell, startup_files(shell).ok()?)))
        .and_then(|(shell, files)| {
            let file = files
                .into_iter()
                .find(|f| has_line(&read(f).unwrap_or_default(), hook_line(shell)))?;
            Some((shell, file))
        });

    match configured {
        Some((shell, file)) => eprintln!(
            "{} 当前终端还没有加载 setproxy（{} 已配置）。\n\n重开终端，或运行:\n\n    {}",
            "✗".red().bold(),
            pretty(&file),
            reload_command(shell, &file)
        ),
        None => eprintln!(
            "{} setproxy 还没有接入当前 shell，无法修改这个终端的环境变量。\n\n先运行一次:\n\n    setproxy init\n\n然后重开终端。",
            "✗".red().bold()
        ),
    }
}

fn install_into(file: &Path, line: &str) -> Result<()> {
    let content = read(file).with_context(|| format!("无法读取 {}", file.display()))?;
    if has_line(&content, line) {
        eprintln!("{} {} 已配置", "✓".green().bold(), pretty(file));
        return Ok(());
    }

    let eol = if content.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let (updated, replaced) = match replace_previous(&content, line, eol) {
        Some(updated) => (updated, true),
        None => (append(&content, line, eol), false),
    };

    if let Some(dir) = file.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(file, updated).with_context(|| format!("无法写入 {}", file.display()))?;
    eprintln!(
        "{} {} {}",
        "✓".green().bold(),
        if replaced { "已更新" } else { "已写入" },
        pretty(file)
    );
    Ok(())
}

/// Replace the hook line an earlier `setproxy init` left under "# setproxy",
/// so re-running init after the line changes updates it in place
fn replace_previous(content: &str, line: &str, eol: &str) -> Option<String> {
    let mut lines: Vec<&str> = content.lines().collect();
    let i = lines.windows(2).position(|w| {
        let next = w[1].trim();
        w[0].trim() == COMMENT
            && next.contains("setproxy")
            && (next.starts_with("eval ") || next.starts_with("Invoke-Expression "))
    })?;
    lines[i + 1] = line;
    let mut out = lines.join(eol);
    out.push_str(eol);
    Some(out)
}

/// Append the hook line, separated from existing content by a blank line
fn append(content: &str, line: &str, eol: &str) -> String {
    let mut out = content.to_string();
    if !out.is_empty() {
        if !out.ends_with('\n') {
            out.push_str(eol);
        }
        out.push_str(eol);
    }
    out.push_str(&format!("{COMMENT}{eol}{line}{eol}"));
    out
}

/// Read a startup file; a missing one is empty
fn read(file: &Path) -> std::io::Result<String> {
    match fs::read_to_string(file) {
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(String::new()),
        result => result,
    }
}

fn has_line(content: &str, line: &str) -> bool {
    content.lines().any(|l| l.trim() == line)
}

/// Startup files that load the integration for this shell
fn startup_files(shell: Shell) -> Result<Vec<PathBuf>> {
    Ok(match shell {
        Shell::Zsh => {
            let dir = env::var_os("ZDOTDIR").map_or_else(home_dir, |d| Ok(PathBuf::from(d)))?;
            vec![dir.join(".zshrc")]
        }
        Shell::Bash => vec![home_dir()?.join(".bashrc")],
        Shell::Powershell => {
            // Windows PowerShell 5.1 and PowerShell 7 keep separate profiles,
            // so ask every installed one for its $PROFILE
            let mut profiles = Vec::new();
            for profile in ["pwsh", "powershell"]
                .into_iter()
                .filter_map(powershell_profile)
            {
                if !profiles.contains(&profile) {
                    profiles.push(profile);
                }
            }
            if profiles.is_empty() {
                anyhow::bail!("找不到 pwsh 或 powershell，无法确定 $PROFILE 的位置");
            }
            profiles
        }
    })
}

fn powershell_profile(exe: &str) -> Option<PathBuf> {
    let output = Command::new(exe)
        .args(["-NoLogo", "-NoProfile", "-NonInteractive", "-Command"])
        .arg("[Console]::OutputEncoding = [Text.Encoding]::UTF8; $PROFILE")
        .output()
        .ok()?;
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (output.status.success() && !path.is_empty()).then(|| PathBuf::from(path))
}

fn reload_command(shell: Shell, file: &Path) -> String {
    match shell {
        Shell::Zsh | Shell::Bash => format!("source {}", pretty(file)),
        Shell::Powershell => ". $PROFILE".to_string(),
    }
}

fn home_dir() -> Result<PathBuf> {
    env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .context("找不到用户主目录（HOME）")
}

/// Path for messages, with the home directory shown as ~
fn pretty(path: &Path) -> String {
    match home_dir()
        .ok()
        .and_then(|home| path.strip_prefix(home).ok().map(Path::to_path_buf))
    {
        Some(rest) => format!("~{}{}", std::path::MAIN_SEPARATOR, rest.display()),
        None => path.display().to_string(),
    }
}

/// The shell `setproxy` was started from. Walks up the process tree, since a
/// pixi trampoline or an install script may sit in between, then falls back
/// to $SHELL (the login shell).
fn detect() -> Option<Shell> {
    ancestor_shell()
        .or_else(|| env::var("SHELL").ok().and_then(|s| from_program(&s)))
        .or_else(|| cfg!(windows).then_some(Shell::Powershell))
}

fn from_program(path: &str) -> Option<Shell> {
    let name = path.rsplit(['/', '\\']).next()?.trim_start_matches('-');
    match name.strip_suffix(".exe").unwrap_or(name) {
        "zsh" => Some(Shell::Zsh),
        "bash" => Some(Shell::Bash),
        "pwsh" | "powershell" => Some(Shell::Powershell),
        _ => None,
    }
}

#[cfg(unix)]
fn ancestor_shell() -> Option<Shell> {
    let mut pid = std::os::unix::process::parent_id();
    for _ in 0..8 {
        if pid <= 1 {
            break;
        }
        let (name, ppid) = process_info(pid)?;
        if let Some(shell) = from_program(&name) {
            return Some(shell);
        }
        pid = ppid;
    }
    None
}

#[cfg(not(unix))]
fn ancestor_shell() -> Option<Shell> {
    None
}

/// (name, parent pid) of a process
#[cfg(target_os = "linux")]
fn process_info(pid: u32) -> Option<(String, u32)> {
    // "pid (comm) state ppid ...", where comm may contain spaces or parentheses
    let stat = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let (open, close) = (stat.find('(')?, stat.rfind(')')?);
    let ppid = stat[close + 1..].split_whitespace().nth(1)?.parse().ok()?;
    Some((stat[open + 1..close].to_string(), ppid))
}

/// (name, parent pid) of a process
#[cfg(all(unix, not(target_os = "linux")))]
fn process_info(pid: u32) -> Option<(String, u32)> {
    let output = Command::new("ps")
        .args(["-o", "ppid=", "-o", "comm=", "-p", &pid.to_string()])
        .output()
        .ok()?;
    let output = String::from_utf8_lossy(&output.stdout);
    let (ppid, name) = output.trim().split_once(char::is_whitespace)?;
    Some((name.trim().to_string(), ppid.parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_program() {
        assert_eq!(from_program("/usr/bin/zsh"), Some(Shell::Zsh));
        assert_eq!(from_program("-bash"), Some(Shell::Bash));
        assert_eq!(
            from_program(r"C:\Program Files\PowerShell\7\pwsh.exe"),
            Some(Shell::Powershell)
        );
        assert_eq!(from_program("sh"), None);
    }

    #[test]
    fn test_replace_previous() {
        let line = hook_line(Shell::Zsh);
        let rc = "export A=1\n# setproxy\neval \"$(setproxy init zsh --print)\"\nexport B=2\n";
        assert_eq!(
            replace_previous(rc, line, "\n").unwrap(),
            format!("export A=1\n# setproxy\n{line}\nexport B=2\n")
        );

        // The user's own lines under a similar comment stay
        let own = "# setproxy\nalias sp='setproxy 1.2.3.4 80'\n";
        assert_eq!(replace_previous(own, line, "\n"), None);
    }

    #[test]
    fn test_append() {
        let line = hook_line(Shell::Bash);
        assert_eq!(append("", line, "\n"), format!("# setproxy\n{line}\n"));
        assert_eq!(
            append("export A=1", line, "\n"),
            format!("export A=1\n\n# setproxy\n{line}\n")
        );
        assert_eq!(
            append("export A=1\r\n", line, "\r\n"),
            format!("export A=1\r\n\r\n# setproxy\r\n{line}\r\n")
        );
    }
}
