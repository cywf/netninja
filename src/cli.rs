use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "netninja-cli")]
#[command(version = "1.0.0")]
#[command(about = "NetNinja - Advanced Linux Network Troubleshooting CLI", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Launch the immersive tmux monitoring dashboard
    Monitor,
    
    /// Show quick network status summary
    Status,
}
