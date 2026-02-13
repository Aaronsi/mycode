//! GBA CLI - Command Line Interface for Geektime Bootcamp Agent
//!
//! This is the main entry point for the GBA command-line tool.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod commands;
mod ui;

#[derive(Parser)]
#[command(name = "gba")]
#[command(
    author,
    version,
    about = "Geektime Bootcamp Agent - AI-assisted feature development"
)]
struct Cli {
    /// Repository path to work with
    #[arg(short, long, default_value = ".", global = true)]
    repo: PathBuf,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize GBA in current repository
    Init {
        /// Force reinitialize even if .gba exists
        #[arg(short, long)]
        force: bool,
    },

    /// Plan a new feature interactively
    Plan {
        /// Feature slug (e.g., "user-auth", "api-v2")
        feature_slug: String,

        /// Initial feature description
        #[arg(short, long)]
        description: Option<String>,
    },

    /// Execute a planned feature
    Run {
        /// Feature slug or ID to execute (e.g., "0001_user-auth" or "user-auth")
        feature: String,

        /// Resume from last checkpoint
        #[arg(short = 'R', long)]
        resume: bool,

        /// Dry run (show what would be executed)
        #[arg(short, long)]
        dry_run: bool,
    },

    /// List features and their status
    List,

    /// Show feature status
    Status {
        /// Feature slug or ID
        feature: Option<String>,
    },

    /// Interactive TUI mode (legacy)
    Tui,

    /// List available prompt templates
    Templates,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let filter = if cli.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    // Resolve repository path
    let repo_path = cli.repo.canonicalize().unwrap_or(cli.repo.clone());

    match cli.command {
        Commands::Init { force } => {
            commands::init::run(&repo_path, force).await?;
        }
        Commands::Plan {
            feature_slug,
            description,
        } => {
            commands::plan::run(&repo_path, &feature_slug, description).await?;
        }
        Commands::Run {
            feature,
            resume,
            dry_run,
        } => {
            commands::run::run(&repo_path, &feature, resume, dry_run).await?;
        }
        Commands::List => {
            commands::list::run(&repo_path)?;
        }
        Commands::Status { feature } => {
            commands::status::run(&repo_path, feature.as_deref())?;
        }
        Commands::Tui => {
            // Legacy TUI mode
            let api_key = std::env::var("ANTHROPIC_API_KEY")
                .context("ANTHROPIC_API_KEY must be set for TUI mode")?;

            let config = gba_core::Config {
                repo_path,
                api_key,
                model: "claude-sonnet-4-5-20250929".to_string(),
                ..Default::default()
            };

            let engine = gba_core::Engine::new(config);
            ui::run_tui(engine).await?;
        }
        Commands::Templates => {
            let prompts_dir = repo_path.join("prompts");
            match gba_pm::PromptManager::new(prompts_dir) {
                Ok(pm) => {
                    println!("Available templates:");
                    match pm.list_templates() {
                        Ok(templates) => {
                            for template in templates {
                                println!("  - {}", template);
                            }
                        }
                        Err(e) => eprintln!("Error listing templates: {}", e),
                    }
                }
                Err(e) => eprintln!("Error initializing prompt manager: {}", e),
            }
        }
    }

    Ok(())
}
