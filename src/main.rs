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
    /// Manage Rust toolchains and versions
    Toolchain {
        #[command(subcommand)]
        action: ToolchainAction,
    },
    /// Initialize new project from templates
    Init {
        /// Project name
        name: Option<String>,
        /// Template to use
        #[arg(short, long)]
        template: Option<String>,
        /// List available templates
        #[arg(long)]
        list_templates: bool,
    },
    /// Analyze and manage dependencies
    Deps {
        #[command(subcommand)]
        action: DepsAction,
    },
    /// GPG signing and verification
    Gpg {
        #[command(subcommand)]
        action: GpgAction,
    },
}

#[derive(Subcommand)]
pub enum ToolchainAction {
    /// List installed toolchains
    List,
    /// Install a toolchain
    Install {
        /// Toolchain to install (stable, beta, nightly, or specific version)
        toolchain: String,
    },
    /// Set default toolchain
    Default {
        /// Toolchain to set as default
        toolchain: String,
    },
    /// Show active toolchain
    Show,
    /// Remove a toolchain
    Remove {
        /// Toolchain to remove
        toolchain: String,
    },
}

#[derive(Subcommand)]
pub enum DepsAction {
    /// Show dependency tree with vulnerabilities
    Tree,
    /// Check for outdated dependencies
    Outdated,
    /// Audit dependencies for security issues
    Audit,
    /// Show dependency licenses
    Licenses,
    /// Analyze dependency sizes
    Size,
}

#[derive(Subcommand)]
pub enum GpgAction {
    /// Sign a release or commit
    Sign {
        /// What to sign (commit, tag, or file)
        target: String,
    },
    /// Verify GPG signature
    Verify {
        /// What to verify
        target: String,
    },
    /// Setup GPG for Rust development
    Setup,
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
        Commands::Toolchain { action } => commands::toolchain::run(action, cli.json).await,
        Commands::Init { name, template, list_templates } => {
            commands::init::run(name, template, list_templates, cli.json).await
        },
        Commands::Deps { action } => commands::deps::run(action, cli.json).await,
        Commands::Gpg { action } => commands::gpg::run(action, cli.json).await,
    }
}
