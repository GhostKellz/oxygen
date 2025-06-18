use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{Level, info};
use tracing_subscriber::fmt;

mod commands;
mod config;
mod utils;

#[derive(Parser)]
#[command(name = "oxy")]
#[command(about = "The essential Rust dev environment enhancer")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, help = "Output in JSON format")]
    json: bool,

    #[arg(short, long, help = "Verbose output")]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run clippy, fmt, and check in sequence
    Check,
    /// Build with enhanced timing and size summaries
    Build,
    /// Diagnose environment and tool issues
    Doctor,
    /// Show current Rust environment information
    Env,
    /// Show project metadata and git status
    Info,
    /// List installed Rust development tools
    Tools,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    let level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    fmt().with_max_level(level).init();

    info!("Starting Oxygen CLI");

    match cli.command {
        Commands::Check => commands::check::run(cli.json).await,
        Commands::Build => commands::build::run(cli.json).await,
        Commands::Doctor => commands::doctor::run(cli.json).await,
        Commands::Env => commands::env::run(cli.json).await,
        Commands::Info => commands::info::run(cli.json).await,
        Commands::Tools => commands::tools::run(cli.json).await,
    }
}
