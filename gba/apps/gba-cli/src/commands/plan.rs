//! Plan command - Plan a new feature interactively

use std::path::Path;

use anyhow::{Result, bail};

use gba_core::FeatureState;

/// Run the plan command
pub async fn run(repo_path: &Path, feature_slug: &str, description: Option<String>) -> Result<()> {
    let gba_path = repo_path.join(".gba");

    // Check if GBA is initialized
    if !gba_path.exists() {
        bail!("GBA not initialized. Run 'gba init' first.");
    }

    // Validate feature slug
    let slug = validate_slug(feature_slug)?;

    // Generate feature ID
    let feature_id = FeatureState::next_feature_id(&gba_path)?;
    let feature_dir_name = format!("{}_{}", feature_id, slug);
    let feature_path = gba_path.join("features").join(&feature_dir_name);

    // Check if feature already exists
    if feature_path.exists() {
        bail!("Feature '{}' already exists.", feature_dir_name);
    }

    println!("Creating feature {}...", feature_dir_name);

    // Create feature directory structure
    std::fs::create_dir_all(feature_path.join("specs"))?;
    std::fs::create_dir_all(feature_path.join("docs"))?;
    println!("✓ Created feature directory");

    // Create initial state
    let state = FeatureState::new(feature_id.clone(), slug.clone());
    state.save(&feature_path)?;
    println!("✓ Created state.yml");

    // Create initial design.md
    let design_content = format!(
        r#"# Feature: {}

## Overview

{}

## Requirements

- [ ] TODO: Add requirements

## Design

### Architecture

TODO: Describe the architecture

### Implementation Plan

1. TODO: Add implementation steps

## Files to Modify

- TODO: List files to create/modify

## Testing Strategy

- TODO: Describe testing approach

## Notes

- Created: {}
"#,
        slug,
        description.as_deref().unwrap_or("TODO: Add description"),
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );

    std::fs::write(feature_path.join("specs/design.md"), design_content)?;
    println!("✓ Created specs/design.md");

    // Create initial verification.md
    let verification_content = format!(
        r#"# Verification Criteria: {}

## Acceptance Criteria

- [ ] TODO: Add acceptance criteria

## Test Cases

### Unit Tests

- [ ] TODO: Add unit test cases

### Integration Tests

- [ ] TODO: Add integration test cases

## Performance Requirements

- TODO: Add performance requirements

## Security Considerations

- TODO: Add security considerations
"#,
        slug
    );

    std::fs::write(
        feature_path.join("specs/verification.md"),
        verification_content,
    )?;
    println!("✓ Created specs/verification.md");

    println!("\nFeature {} created successfully!", feature_dir_name);
    println!("\nNext steps:");
    println!(
        "  1. Edit .gba/features/{}/specs/design.md",
        feature_dir_name
    );
    println!(
        "  2. Edit .gba/features/{}/specs/verification.md",
        feature_dir_name
    );
    println!(
        "  3. Run 'gba run {}' to execute the feature",
        feature_dir_name
    );

    Ok(())
}

/// Validate and normalize a feature slug
fn validate_slug(slug: &str) -> Result<String> {
    // Convert to lowercase and replace spaces/underscores with hyphens
    let normalized: String = slug
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();

    // Remove consecutive hyphens and trim
    let cleaned: String = normalized
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    if cleaned.is_empty() {
        bail!("Invalid feature slug: '{}'", slug);
    }

    Ok(cleaned)
}
