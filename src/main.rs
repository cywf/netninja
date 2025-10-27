mod cli;
mod monitor;
mod tmux;
mod network;
mod security;

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();
    
    match args.command {
        Some(cli::Commands::Monitor) => {
            // Launch the tmux-based monitoring dashboard
            monitor::launch_dashboard().await?;
        }
        Some(cli::Commands::Status) => {
            // Show quick network status
            monitor::show_status().await?;
        }
        None => {
            // Default: show help
            cli::Args::parse_from(&["netninja-cli", "--help"]);
        }
    }
    
    Ok(())
}
