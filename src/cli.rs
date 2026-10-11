use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "_setproxy")]
#[command(author, version, about = "A cross-platform CLI tool to manage proxy environment variables", long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
#[command(subcommand_precedence_over_arg = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Proxy host (auto-detect WSL2 host if not specified)
    #[arg(value_name = "HOST")]
    pub host: Option<String>,

    /// Proxy port
    #[arg(value_name = "PORT")]
    pub port: Option<u16>,
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
}
