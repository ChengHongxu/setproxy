use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "setproxy")]
#[command(author, version, about = "A cross-platform CLI tool to manage proxy environment variables", long_about = None)]
#[command(subcommand_precedence_over_arg = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Set by the shell function: syntax of the code to print for it to evaluate
    #[arg(long, value_enum, global = true, hide = true)]
    pub syntax: Option<Syntax>,

    /// Proxy host (auto-detect WSL2 host if not specified)
    #[arg(value_name = "HOST")]
    pub host: Option<String>,

    /// Proxy port
    #[arg(value_name = "PORT")]
    pub port: Option<u16>,
}

/// Language of the code printed on stdout
#[derive(Clone, Copy, ValueEnum)]
pub enum Syntax {
    /// bash / zsh
    Posix,
    Powershell,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Shell {
    Zsh,
    Bash,
    #[value(alias = "pwsh")]
    Powershell,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Clear all proxy settings
    #[command(alias = "c")]
    Clear,

    /// Show current proxy status
    #[command(alias = "s")]
    Status,

    /// Set proxy using WSL2 auto-detection
    Wsl {
        /// Proxy port (default: 8090)
        #[arg(default_value = "8090")]
        port: u16,
    },

    /// Set up shell integration in your shell's startup file (run once)
    Init {
        /// Shell to set up; detected from the current shell if omitted
        #[arg(value_enum)]
        shell: Option<Shell>,

        /// Print the shell integration instead (this is what the startup file runs)
        #[arg(long)]
        print: bool,
    },
}
