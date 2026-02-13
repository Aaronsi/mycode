//! Init command - Initialize GBA in a repository

use std::path::Path;

use anyhow::{Context, Result, bail};

/// Default configuration content
const DEFAULT_CONFIG: &str = r#"# GBA Configuration
version: "0.1.0"

# Agent configuration
agent:
  # API key environment variable name
  apiKeyEnv: "ANTHROPIC_API_KEY"
  # Default Claude model
  model: "claude-sonnet-4-5"
  # Permission mode: default | acceptEdits | plan | bypassPermissions
  permissionMode: "default"
  # Timeout in seconds
  timeoutSeconds: 300
  # Maximum turns per phase
  maxTurns: 50

# Git configuration
git:
  # Auto-commit after each phase
  autoCommit: true
  # Branch naming pattern (variables: {id}, {slug})
  branchPattern: "feature/{id}-{slug}"
  # Use git worktree for isolation
  useWorktree: true
  # Base branch for new features
  baseBranch: "main"

# Phase execution order
phases:
  - name: "observe"
    description: "Observe codebase and understand context"
  - name: "build"
    description: "Build implementation"
  - name: "test"
    description: "Write and run tests"
  - name: "verification"
    description: "Verify implementation against requirements"
  - name: "review"
    description: "Code review and refinement"
  - name: "pr"
    description: "Create pull request"
"#;

/// Run the init command
pub async fn run(repo_path: &Path, force: bool) -> Result<()> {
    let gba_path = repo_path.join(".gba");
    let trees_path = repo_path.join(".trees");

    // Check if already initialized
    if gba_path.exists() && !force {
        bail!("GBA already initialized in this repository. Use --force to reinitialize.");
    }

    println!("Initializing GBA for current project...");

    // Create .gba directory structure
    std::fs::create_dir_all(&gba_path).context("Failed to create .gba directory")?;
    std::fs::create_dir_all(gba_path.join("features"))
        .context("Failed to create features directory")?;
    println!("✓ Created .gba/ directory");

    // Create .trees directory for worktrees
    std::fs::create_dir_all(&trees_path).context("Failed to create .trees directory")?;
    println!("✓ Created .trees/ directory");

    // Create default config.yml
    let config_path = gba_path.join("config.yml");
    std::fs::write(&config_path, DEFAULT_CONFIG).context("Failed to write config.yml")?;
    println!("✓ Created config.yml");

    // Update .gitignore
    update_gitignore(repo_path)?;
    println!("✓ Updated .gitignore");

    // Create .gba.md if it doesn't exist
    let gba_md_path = repo_path.join(".gba.md");
    if !gba_md_path.exists() {
        let gba_md_content = r#"# GBA Repository Documentation

This repository is configured for use with GBA (Geektime Bootcamp Agent).

## Directory Structure

- `.gba/` - GBA configuration and feature data
  - `config.yml` - GBA configuration
  - `features/` - Feature-specific data
- `.trees/` - Git worktrees for feature isolation (gitignored)
- `prompts/` - Prompt templates for AI phases

## Usage

```bash
# Initialize GBA (already done)
gba init

# Plan a new feature
gba plan <feature-slug>

# Execute a planned feature
gba run <feature>

# List features
gba list

# Show feature status
gba status [feature]
```

## Configuration

See `.gba/config.yml` for configuration options.
"#;
        std::fs::write(&gba_md_path, gba_md_content).context("Failed to write .gba.md")?;
        println!("✓ Created .gba.md");
    }

    println!("\nDone! Project initialized.");
    println!("Next steps:");
    println!("  1. Review .gba/config.yml and adjust settings");
    println!("  2. Create prompt templates in prompts/ directory");
    println!("  3. Run 'gba plan <feature-slug>' to plan a new feature");

    Ok(())
}

/// Update .gitignore to include GBA-specific entries
fn update_gitignore(repo_path: &Path) -> Result<()> {
    let gitignore_path = repo_path.join(".gitignore");

    let entries_to_add = vec![
        "# GBA (Geektime Bootcamp Agent)",
        ".trees/",
        ".gba/features/*/trees/",
    ];

    let existing_content = if gitignore_path.exists() {
        std::fs::read_to_string(&gitignore_path)?
    } else {
        String::new()
    };

    let mut new_entries = Vec::new();
    for entry in entries_to_add {
        if !existing_content.contains(entry) {
            new_entries.push(entry);
        }
    }

    if !new_entries.is_empty() {
        let mut content = existing_content;
        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }
        content.push('\n');
        content.push_str(&new_entries.join("\n"));
        content.push('\n');

        std::fs::write(&gitignore_path, content)?;
    }

    Ok(())
}
