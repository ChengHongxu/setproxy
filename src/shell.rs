use crate::cli::Shell;
use anyhow::{Context, Result};
use colored::*;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;

const POSIX_INIT: &str = include_str!("../shell/init.sh");
const POWERSHELL_INIT: &str = include_str!("../shell/init.ps1");

/// Comment written above the init line
const COMMENT: &str = "# setproxy";

/// Shell integration code printed by `setproxy init <shell> --print`
pub fn init_script(shell: Shell) -> &'static str {
    match shell {
        Shell::Zsh | Shell::Bash => POSIX_INIT,
        Shell::Powershell => POWERSHELL_INIT,
    }
}

/// Line that `setproxy init` adds to the startup file
fn init_line(shell: Shell) -> &'static str {
    match shell {
        Shell::Zsh => r#"eval "$(setproxy init zsh --print)""#,
        Shell::Bash => r#"eval "$(setproxy init bash --print)""#,
        Shell::Powershell => "Invoke-Expression (& setproxy init powershell --print | Out-String)",
    }
}

pub fn resolve(shell: Option<Shell>) -> Result<Shell> {
    shell
        .or_else(detect)
        .context("无法识别当前 shell，请指定: setproxy init <zsh|bash|pwsh>")
}

/// `setproxy init`: add the init line to the shell's startup file(s)
pub fn install(shell: Shell) -> Result<()> {
    let files = startup_files(shell)?;
    for file in &files {
        install_into(file, init_line(shell))?;
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
                .find(|f| has_line(&read(f).unwrap_or_default(), init_line(shell)))?;
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
    let eol = if content.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let (mut updated, removed) = strip_legacy(&content, eol);
    let present = has_line(&updated, line);

    if present && removed.is_empty() {
        eprintln!("{} {} 已配置", "✓".green().bold(), pretty(file));
        return Ok(());
    }

    if !present {
        if !updated.is_empty() {
            if !updated.ends_with('\n') {
                updated.push_str(eol);
            }
            updated.push_str(eol);
        }
        updated.push_str(&format!("{COMMENT}{eol}{line}{eol}"));
    }

    if let Some(dir) = file.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(file, updated).with_context(|| format!("无法写入 {}", file.display()))?;

    for old in &removed {
        eprintln!("{} 已移除旧版配置: {}", "-".yellow().bold(), old);
    }
    eprintln!(
        "{} {} {}",
        "✓".green().bold(),
        if present { "已更新" } else { "已写入" },
        pretty(file)
    );
    Ok(())
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

/// Lines written by setproxy 0.2.x (whose binary was `_setproxy`) or its
/// installers. Matched narrowly so the user's own lines are never touched.
fn is_legacy(line: &str) -> bool {
    let line = line.trim();
    let sources = |name: &str| {
        (line.starts_with("source ") || line.starts_with(". "))
            && line.trim_end_matches(['"', '\'']).ends_with(name)
    };
    line == "# >>> setproxy initialized <<<"
        || line == "# setproxy - source shell function"
        || line.starts_with(r#"eval "$(_setproxy "#)
        || line.starts_with("Invoke-Expression (& _setproxy ")
        || sources("setproxy.sh")
        || sources("setproxy.ps1")
        || sources("$(which setproxy)")
}

/// Remove legacy lines (and a "# setproxy" comment right above one).
/// Returns the content unchanged when there is nothing to remove.
fn strip_legacy(content: &str, eol: &str) -> (String, Vec<String>) {
    let mut kept: Vec<&str> = Vec::new();
    let mut removed = Vec::new();
    for line in content.lines() {
        if is_legacy(line) {
            removed.push(line.trim().to_string());
            if kept.last().is_some_and(|l| l.trim() == COMMENT) {
                kept.pop();
            }
        } else {
            kept.push(line);
        }
    }
    if removed.is_empty() {
        return (content.to_string(), removed);
    }

    while kept.last().is_some_and(|l| l.trim().is_empty()) {
        kept.pop();
    }
    let mut out = kept.join(eol);
    if !out.is_empty() {
        out.push_str(eol);
    }
    (out, removed)
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
    fn test_strip_legacy() {
        let rc = concat!(
            "export A=1\n",
            "\n",
            "# >>> setproxy initialized <<<\n",
            "eval \"$(_setproxy shell-init)\"\n",
            "alias sp='setproxy 1.2.3.4 80'\n",
            "# setproxy\n",
            ". \"C:\\x\\setproxy.ps1\"\n",
            "\n",
        );
        let (kept, removed) = strip_legacy(rc, "\n");
        assert_eq!(kept, "export A=1\n\nalias sp='setproxy 1.2.3.4 80'\n");
        assert_eq!(removed.len(), 3);

        // The user's own lines that merely mention setproxy stay
        let own = "source ~/my-setproxy-notes.txt\nexport X=_setproxy\n";
        assert_eq!(strip_legacy(own, "\n"), (own.to_string(), vec![]));

        let current = format!("{COMMENT}\n{}\n", init_line(Shell::Zsh));
        assert_eq!(strip_legacy(&current, "\n"), (current.clone(), vec![]));
    }
}
