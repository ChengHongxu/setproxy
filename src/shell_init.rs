use anyhow::Result;
use colored::*;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

const INIT_MARKER: &str = "# >>> setproxy initialized <<<";
const SHELL_INIT_LINE: &str = r#"eval "$(_setproxy shell-init)""#;

/// Check if setproxy shell integration is configured
pub fn is_shell_configured() -> bool {
    if let Some(shell_rc) = get_shell_rc() {
        if let Ok(content) = fs::read_to_string(&shell_rc) {
            // Check if already configured (either has the marker or the eval line)
            return content.contains(INIT_MARKER) || content.contains(SHELL_INIT_LINE);
        }
    }
    false
}

/// Auto-configure shell integration by adding eval line to shell RC
pub fn auto_configure_shell() -> Result<()> {
    let shell_rc = get_shell_rc()
        .ok_or_else(|| anyhow::anyhow!("Could not detect shell RC file (.zshrc or .bashrc)"))?;

    // Check if already configured
    if let Ok(content) = fs::read_to_string(&shell_rc) {
        if content.contains(INIT_MARKER) || content.contains(SHELL_INIT_LINE) {
            return Ok(()); // Already configured, silent success
        }
    }

    // Append configuration
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&shell_rc)?;

    writeln!(file)?;
    writeln!(file, "{}", INIT_MARKER)?;
    writeln!(file, "{}", SHELL_INIT_LINE)?;

    eprintln!(
        "{} Shell integration configured in {}",
        "✓".green().bold(),
        shell_rc.display()
    );
    eprintln!(
        "{} Run: {}",
        "→".cyan(),
        format!("source {}", shell_rc.display()).yellow()
    );

    Ok(())
}

/// Detect shell RC file based on current shell
fn get_shell_rc() -> Option<PathBuf> {
    let home = env::var("HOME").ok()?;
    let shell = env::var("SHELL").ok()?;

    let rc_file = if shell.contains("zsh") {
        ".zshrc"
    } else if shell.contains("bash") {
        ".bashrc"
    } else {
        return None;
    };

    Some(PathBuf::from(home).join(rc_file))
}
