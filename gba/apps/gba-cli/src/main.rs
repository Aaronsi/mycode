use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod ui;

#[derive(Parser)]
#[command(name = "gba")]
#[command(author, version, about = "Geektime Bootcamp Agent - A CLI tool for Claude Agent SDK", long_about = None)]
struct Cli {
    /// Repository path to work with
    #[arg(short, long, default_value = ".")]
    repo: PathBuf,

    /// Claude API key (or set ANTHROPIC_API_KEY env var)
    #[arg(short, long, env)]
    api_key: Option<String>,

    /// Model to use
    #[arg(short, long, default_value = "claude-sonnet-4-5-20250929")]
    model: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a task with a prompt
    Execute {
        /// The prompt to execute
        prompt: String,
    },
    /// Interactive TUI mode
    Tui,
    /// List available prompt templates
    Templates,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Get API key from args or environment
    let api_key = cli.api_key.unwrap_or_else(|| {
        std::env::var("ANTHROPIC_API_KEY")
            .expect("ANTHROPIC_API_KEY must be set either via --api-key or environment variable")
    });

    // Create core engine config
    let config = gba_core::Config {
        repo_path: cli.repo,
        api_key,
        model: cli.model,
        ..Default::default()
    };

    let engine = gba_core::Engine::new(config);

    match cli.command {
        Commands::Execute { prompt } => {
            println!("Executing prompt: {}", prompt);
            let result = engine.execute(&prompt).await?;
            println!("Result: {}", result);
        }
        Commands::Tui => {
            println!("Starting TUI mode...");
            ui::run_tui(engine).await?;
        }
        Commands::Templates => {
            // Look for prompts directory relative to repo path
            let prompts_dir = engine.config().repo_path.join("prompts");
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
