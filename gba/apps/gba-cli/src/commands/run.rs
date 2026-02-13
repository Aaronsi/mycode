//! Run command - Execute a planned feature

use std::path::Path;

use anyhow::{Context, Result, bail};

use gba_core::{Config, Engine, ExecutionContext, FeatureState, FeatureStatus, Phase, PhaseStatus};

/// Run the run command
pub async fn run(repo_path: &Path, feature: &str, resume: bool, dry_run: bool) -> Result<()> {
    let gba_path = repo_path.join(".gba");

    // Check if GBA is initialized
    if !gba_path.exists() {
        bail!("GBA not initialized. Run 'gba init' first.");
    }

    // Find the feature
    let feature_path = find_feature(&gba_path, feature)?;
    let mut state = FeatureState::load(&feature_path)?;

    println!(
        "Feature: {}_{} (Status: {:?})",
        state.feature.id, state.feature.slug, state.status
    );

    // Check if we can run
    if state.status == FeatureStatus::Completed {
        println!("Feature already completed.");
        return Ok(());
    }

    // Determine starting phase
    let start_phase = if resume && state.resume.can_resume {
        println!(
            "Resuming from phase: {}",
            state.resume.next_phase.as_deref().unwrap_or("unknown")
        );
        state.current_phase
    } else if state.status == FeatureStatus::InProgress {
        println!("Feature is in progress. Use --resume to continue.");
        return Ok(());
    } else {
        0
    };

    if dry_run {
        println!("\n[DRY RUN] Would execute the following phases:");
        let phases = get_default_phases();
        for (i, phase) in phases.iter().enumerate().skip(start_phase) {
            println!("  {}. {} - {}", i + 1, phase.0, phase.1);
        }
        return Ok(());
    }

    // Get API key
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .context("ANTHROPIC_API_KEY must be set to run features")?;

    // Create engine
    let config = Config {
        repo_path: repo_path.to_path_buf(),
        api_key,
        model: "claude-sonnet-4-5-20250929".to_string(),
        ..Default::default()
    };

    let engine = Engine::new(config);

    // Mark as in progress
    state.start_execution();
    state.save(&feature_path)?;

    // Get phases to execute
    let phase_defs = get_default_phases();
    let phases: Vec<Phase> = phase_defs
        .iter()
        .skip(start_phase)
        .map(|(name, desc)| Phase {
            name: name.to_string(),
            description: desc.to_string(),
            preset: true,
            tools: vec![],
            disallowed_tools: vec![],
            context: ExecutionContext {
                repo_path: repo_path.to_path_buf(),
                feature_slug: state.feature.slug.clone(),
                feature_id: state.feature.id.clone(),
                phase_name: Some(name.to_string()),
                ..Default::default()
            },
        })
        .collect();

    println!("\nExecuting {} phases...\n", phases.len());

    // Execute phases
    for (i, phase) in phases.iter().enumerate() {
        let phase_idx = start_phase + i;
        println!(
            "Phase {}/{}: {}",
            phase_idx + 1,
            phase_defs.len(),
            phase.name
        );

        // Update state
        state.current_phase = phase_idx;
        state.update_phase(&phase.name, PhaseStatus::InProgress, None, None);
        state.save(&feature_path)?;

        // Execute phase
        match engine
            .execute(&format!(
                "Execute phase '{}' for feature '{}'.\n\nDescription: {}\n\nRead the design spec at .gba/features/{}_{}/specs/design.md and implement accordingly.",
                phase.name,
                state.feature.slug,
                phase.description,
                state.feature.id,
                state.feature.slug
            ))
            .await
        {
            Ok(output) => {
                println!("  ✓ Phase completed");

                // Update state
                let stats = gba_core::ExecutionStats::default();
                state.update_phase(
                    &phase.name,
                    PhaseStatus::Completed,
                    Some(&stats),
                    Some(truncate_output(&output, 200)),
                );
                state.save(&feature_path)?;
            }
            Err(e) => {
                println!("  ✗ Phase failed: {}", e);

                // Update state
                state.update_phase(&phase.name, PhaseStatus::Failed, None, Some(e.to_string()));
                state.fail(e.to_string());
                state.save(&feature_path)?;

                bail!("Phase '{}' failed: {}", phase.name, e);
            }
        }
    }

    // Mark as completed
    state.complete(None);
    state.save(&feature_path)?;

    println!("\n✓ Feature execution completed!");
    println!("  Total phases: {}", phase_defs.len());
    println!("  Total cost: ${:.4}", state.total_stats.cost_usd);

    Ok(())
}

/// Find a feature by slug or ID
fn find_feature(gba_path: &Path, feature: &str) -> Result<std::path::PathBuf> {
    let features_path = gba_path.join("features");

    if !features_path.exists() {
        bail!("No features found. Run 'gba plan <feature-slug>' first.");
    }

    // Try exact match first
    let exact_path = features_path.join(feature);
    if exact_path.exists() {
        return Ok(exact_path);
    }

    // Try to find by slug suffix
    for entry in std::fs::read_dir(&features_path)? {
        let entry = entry?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Check if name ends with the feature slug
        if name_str.ends_with(&format!("_{}", feature)) {
            return Ok(entry.path());
        }

        // Check if name starts with the feature ID
        if name_str.starts_with(&format!("{}_", feature)) {
            return Ok(entry.path());
        }
    }

    bail!("Feature '{}' not found.", feature);
}

/// Get default phase definitions
fn get_default_phases() -> Vec<(&'static str, &'static str)> {
    vec![
        ("observe", "Observe codebase and understand context"),
        ("build", "Build implementation"),
        ("test", "Write and run tests"),
        ("verification", "Verify implementation against requirements"),
        ("review", "Code review and refinement"),
        ("pr", "Create pull request"),
    ]
}

/// Truncate output to a maximum length
fn truncate_output(output: &str, max_len: usize) -> String {
    if output.len() <= max_len {
        output.to_string()
    } else {
        format!("{}...", &output[..max_len])
    }
}
