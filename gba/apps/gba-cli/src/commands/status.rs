//! Status command - Show feature status

use std::path::Path;

use anyhow::{Result, bail};

use gba_core::FeatureState;

/// Run the status command
pub fn run(repo_path: &Path, feature: Option<&str>) -> Result<()> {
    let gba_path = repo_path.join(".gba");
    let features_path = gba_path.join("features");

    // Check if GBA is initialized
    if !gba_path.exists() {
        bail!("GBA not initialized. Run 'gba init' first.");
    }

    match feature {
        Some(feature_name) => {
            // Show specific feature status
            let feature_path = find_feature(&features_path, feature_name)?;
            let state = FeatureState::load(&feature_path)?;
            print_feature_status(&state);
        }
        None => {
            // Show summary of all features
            if !features_path.exists() {
                println!("No features found.");
                return Ok(());
            }

            let mut total = 0;
            let mut planned = 0;
            let mut in_progress = 0;
            let mut completed = 0;
            let mut failed = 0;

            for entry in std::fs::read_dir(&features_path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir()
                    && let Ok(state) = FeatureState::load(&path)
                {
                    total += 1;
                    match state.status {
                        gba_core::FeatureStatus::Planned => planned += 1,
                        gba_core::FeatureStatus::InProgress => in_progress += 1,
                        gba_core::FeatureStatus::Completed => completed += 1,
                        gba_core::FeatureStatus::Failed => failed += 1,
                    }
                }
            }

            println!("Feature Summary:");
            println!("  Total:       {}", total);
            println!("  Planned:     {}", planned);
            println!("  In Progress: {}", in_progress);
            println!("  Completed:   {}", completed);
            println!("  Failed:      {}", failed);
        }
    }

    Ok(())
}

/// Find a feature by slug or ID
fn find_feature(features_path: &Path, feature: &str) -> Result<std::path::PathBuf> {
    if !features_path.exists() {
        bail!("No features found.");
    }

    // Try exact match first
    let exact_path = features_path.join(feature);
    if exact_path.exists() {
        return Ok(exact_path);
    }

    // Try to find by slug suffix or ID prefix
    for entry in std::fs::read_dir(features_path)? {
        let entry = entry?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if name_str.ends_with(&format!("_{}", feature))
            || name_str.starts_with(&format!("{}_", feature))
        {
            return Ok(entry.path());
        }
    }

    bail!("Feature '{}' not found.", feature);
}

/// Print detailed feature status
fn print_feature_status(state: &FeatureState) {
    println!("Feature: {}_{}", state.feature.id, state.feature.slug);
    println!("Status:  {:?}", state.status);
    println!(
        "Created: {}",
        state.feature.created_at.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!(
        "Updated: {}",
        state.feature.updated_at.format("%Y-%m-%d %H:%M:%S UTC")
    );

    if let Some(ref git) = state.git {
        println!("\nGit:");
        println!("  Branch:     {}", git.branch);
        println!("  Base:       {}", git.base_branch);
        println!("  Worktree:   {}", git.worktree_path);
    }

    if !state.phases.is_empty() {
        println!("\nPhases:");
        for (i, phase) in state.phases.iter().enumerate() {
            let status_icon = match phase.status {
                gba_core::PhaseStatus::Pending => "○",
                gba_core::PhaseStatus::InProgress => "◐",
                gba_core::PhaseStatus::Completed => "●",
                gba_core::PhaseStatus::Failed => "✗",
            };
            let current = if i == state.current_phase { " ←" } else { "" };
            println!(
                "  {} {} {:?}{}",
                status_icon, phase.name, phase.status, current
            );

            if let Some(ref summary) = phase.output_summary {
                println!("    Summary: {}", summary);
            }
            if let Some(ref stats) = phase.stats {
                println!("    Stats: {} turns, ${:.4}", stats.turns, stats.cost_usd);
            }
        }
    }

    println!("\nTotal Stats:");
    println!("  Turns: {}", state.total_stats.turns);
    println!("  Cost:  ${:.4}", state.total_stats.cost_usd);

    if state.resume.can_resume {
        println!("\nResume Info:");
        if let Some(ref last) = state.resume.last_completed_phase {
            println!("  Last completed: {}", last);
        }
        if let Some(ref next) = state.resume.next_phase {
            println!("  Next phase:     {}", next);
        }
        if let Some(ref reason) = state.resume.interrupt_reason {
            println!("  Interrupt:      {:?}", reason);
        }
    }

    if let Some(ref pr) = state.pull_request {
        println!("\nPull Request:");
        if let Some(ref url) = pr.url {
            println!("  URL:    {}", url);
        }
        if let Some(number) = pr.number {
            println!("  Number: #{}", number);
        }
        println!("  Merged: {}", pr.merged);
    }

    if let Some(ref error) = state.error {
        println!("\nError: {}", error);
    }
}
