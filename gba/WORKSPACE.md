# GBA Workspace Structure

This project has been converted to a Cargo workspace with the following structure:

## Workspace Layout

```
gba/
├── Cargo.toml                 # Workspace root configuration
├── apps/
│   └── gba-cli/              # Command line interface
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs       # CLI entry point with clap
│           └── ui.rs         # TUI implementation with ratatui
├── crates/
│   ├── gba-core/             # Core execution engine
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs        # Engine with tokio + claude-agent-sdk-rs
│   └── gba-pm/               # Prompt manager
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs        # Template manager with minijinja
```

## Crates

### `apps/gba-cli`
Command line interface built with:
- **clap**: Command-line argument parsing
- **ratatui**: Terminal UI framework
- **crossterm**: Cross-platform terminal manipulation

Features:
- Execute prompts directly
- Interactive TUI mode
- List available templates

### `crates/gba-core`
Core execution engine built with:
- **tokio**: Async runtime
- **claude-agent-sdk-rs 0.6**: Claude Agent SDK integration
- **serde/serde_json**: Serialization

Provides the main execution engine for running Claude agent tasks.

### `crates/gba-pm`
Prompt manager built with:
- **minijinja**: Template engine
- **serde/serde_json**: Serialization

Manages prompt templates with variable substitution.

## Dependencies

All dependencies are defined at the workspace level in the root `Cargo.toml` and referenced using `{workspace = true}` in individual crates.

## Building

```bash
# Build entire workspace
cargo build

# Build specific crate
cargo build -p gba-cli
cargo build -p gba-core
cargo build -p gba-pm

# Run the CLI
cargo run -p gba-cli -- --help
```

## Usage

```bash
# Execute a prompt
gba execute "your prompt here"

# Start interactive TUI
gba tui

# List templates
gba templates
```
