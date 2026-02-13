//! List command - List features and their status

use std::path::Path;

use anyhow::{Result, bail};

use gba_core::FeatureState;

/// Run the list command
pub fn run(repo_path: &Path) -> Result<()> {
    let gba_path = repo_path.join(".gba");
    let features_path = gba_path.join("features");

    // Check if GBA is initialized
    if !gba_path.exists() {
        bail!("GBA not initialized. Run 'gba init' first.");
    }

    if !features_path.exists() {
        println!("No features found.");
        println!("Run 'gba plan <feature-slug>' to create a new feature.");
        return Ok(());
    }

    // Collect features
    let mut features = Vec::new();

    for entry in std::fs::read_dir(&features_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir()
            && let Ok(state) = FeatureState::load(&path)
        {
            features.push((entry.file_name().to_string_lossy().to_string(), state));
        }
    }

    if features.is_empty() {
        println!("No features found.");
        println!("Run 'gba plan <feature-slug>' to create a new feature.");
        return Ok(());
    }

    // Sort by ID
    features.sort_by(|a, b| a.1.feature.id.cmp(&b.1.feature.id));

    // Print header
    println!(
        "{:<20} {:<15} {:<10} {:<20}",
        "Feature", "Status", "Phase", "Updated"
    );
    println!("{}", "-".repeat(70));

    // Print features
    for (name, state) in features {
        let status = format!("{:?}", state.status);
        let phase = if state.phases.is_empty() {
            "-".to_string()
        } else {
            format!("{}/{}", state.current_phase + 1, state.phases.len())
        };
        let updated = state
            .feature
            .updated_at
            .format("%Y-%m-%d %H:%M")
            .to_string();

        println!("{:<20} {:<15} {:<10} {:<20}", name, status, phase, updated);
    }

    Ok(())
}
