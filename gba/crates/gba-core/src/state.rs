//! Feature state management for GBA
//!
//! This module provides types and functions for managing feature execution state,
//! including persistence to state.yml files.

use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{CoreError, ExecutionStats, Result};

/// Feature state - tracks execution progress and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureState {
    /// State file version
    pub version: String,

    /// Feature information
    pub feature: FeatureInfo,

    /// Current status
    pub status: FeatureStatus,

    /// Current phase index (0-based)
    pub current_phase: usize,

    /// Git information (if using worktree)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git: Option<GitInfo>,

    /// Phase execution history
    pub phases: Vec<PhaseState>,

    /// Total statistics (accumulated across all phases)
    pub total_stats: ExecutionStats,

    /// Execution timing
    pub execution: ExecutionTiming,

    /// Pull request information (populated after PR phase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequestInfo>,

    /// Resume information (for interrupted executions)
    pub resume: ResumeInfo,

    /// Error information (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Default for FeatureState {
    fn default() -> Self {
        Self {
            version: "0.1.0".to_string(),
            feature: FeatureInfo::default(),
            status: FeatureStatus::Planned,
            current_phase: 0,
            git: None,
            phases: vec![],
            total_stats: ExecutionStats::default(),
            execution: ExecutionTiming::default(),
            pull_request: None,
            resume: ResumeInfo::default(),
            error: None,
        }
    }
}

impl FeatureState {
    /// Create a new feature state
    pub fn new(feature_id: String, feature_slug: String) -> Self {
        Self {
            feature: FeatureInfo {
                id: feature_id,
                slug: feature_slug,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            ..Default::default()
        }
    }

    /// Load state from state.yml file
    pub fn load(feature_path: &Path) -> Result<Self> {
        let state_path = feature_path.join("state.yml");
        if !state_path.exists() {
            return Err(CoreError::FeatureNotFound(
                feature_path.display().to_string(),
            ));
        }

        let content = std::fs::read_to_string(&state_path)?;
        let state: FeatureState =
            serde_yaml::from_str(&content).map_err(|e| CoreError::Serialization(e.to_string()))?;

        Ok(state)
    }

    /// Save state to state.yml file
    pub fn save(&self, feature_path: &Path) -> Result<()> {
        let state_path = feature_path.join("state.yml");

        // Create directory if it doesn't exist
        if let Some(parent) = state_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content =
            serde_yaml::to_string(self).map_err(|e| CoreError::Serialization(e.to_string()))?;

        std::fs::write(&state_path, content)?;
        Ok(())
    }

    /// Update phase state with execution result
    pub fn update_phase(
        &mut self,
        phase_name: &str,
        status: PhaseStatus,
        stats: Option<&ExecutionStats>,
        output_summary: Option<String>,
    ) {
        self.feature.updated_at = Utc::now();

        // Find or create phase state
        if let Some(phase) = self.phases.iter_mut().find(|p| p.name == phase_name) {
            phase.status = status;
            if status == PhaseStatus::InProgress {
                phase.started_at = Some(Utc::now());
            } else if status == PhaseStatus::Completed || status == PhaseStatus::Failed {
                phase.completed_at = Some(Utc::now());
            }
            if let Some(s) = stats {
                phase.stats = Some(s.clone());
                // Accumulate total stats
                self.total_stats.turns += s.turns;
                self.total_stats.input_tokens += s.input_tokens;
                self.total_stats.output_tokens += s.output_tokens;
                self.total_stats.cost_usd += s.cost_usd;
            }
            phase.output_summary = output_summary;
        } else {
            // Create new phase state
            let mut phase = PhaseState {
                name: phase_name.to_string(),
                status,
                started_at: None,
                completed_at: None,
                commit_sha: None,
                output_summary,
                stats: stats.cloned(),
            };
            if status == PhaseStatus::InProgress {
                phase.started_at = Some(Utc::now());
            }
            self.phases.push(phase);
        }
    }

    /// Set commit SHA for a phase
    pub fn set_phase_commit(&mut self, phase_name: &str, commit_sha: String) {
        if let Some(phase) = self.phases.iter_mut().find(|p| p.name == phase_name) {
            phase.commit_sha = Some(commit_sha);
        }
    }

    /// Mark feature as completed
    pub fn complete(&mut self, pr_info: Option<PullRequestInfo>) {
        self.status = FeatureStatus::Completed;
        self.execution.end_time = Some(Utc::now());
        self.pull_request = pr_info;
        self.resume.can_resume = false;
        self.feature.updated_at = Utc::now();
    }

    /// Mark feature as failed
    pub fn fail(&mut self, error: String) {
        self.status = FeatureStatus::Failed;
        self.error = Some(error);
        self.resume.can_resume = true;
        self.feature.updated_at = Utc::now();
    }

    /// Mark for resume after interruption
    pub fn mark_for_resume(&mut self, reason: InterruptReason) {
        self.resume.can_resume = true;
        self.resume.interrupted_at = Some(Utc::now());
        self.resume.interrupt_reason = Some(reason);

        // Find last completed phase
        let completed_phases: Vec<_> = self
            .phases
            .iter()
            .filter(|p| p.status == PhaseStatus::Completed)
            .map(|p| p.name.clone())
            .collect();

        self.resume.last_completed_phase = completed_phases.last().cloned();

        // Find next phase
        if self.current_phase < self.phases.len() {
            self.resume.next_phase = Some(self.phases[self.current_phase].name.clone());
        }

        self.feature.updated_at = Utc::now();
    }

    /// Start execution
    pub fn start_execution(&mut self) {
        self.status = FeatureStatus::InProgress;
        self.execution.start_time = Some(Utc::now());
        self.feature.updated_at = Utc::now();
    }

    /// Get next feature ID by scanning existing features
    pub fn next_feature_id(gba_path: &Path) -> Result<String> {
        let features_path = gba_path.join("features");
        if !features_path.exists() {
            return Ok("0001".to_string());
        }

        let mut max_id = 0u32;

        for entry in std::fs::read_dir(&features_path)? {
            let entry = entry?;
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            // Parse ID from directory name (format: NNNN_slug)
            if let Some(id_str) = name_str.split('_').next()
                && let Ok(id) = id_str.parse::<u32>()
            {
                max_id = max_id.max(id);
            }
        }

        Ok(format!("{:04}", max_id + 1))
    }
}

/// Feature identification information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FeatureInfo {
    /// Feature ID (e.g., "0001")
    pub id: String,

    /// Feature slug (e.g., "user-authentication")
    pub slug: String,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Feature execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum FeatureStatus {
    /// Feature is planned but not started
    #[default]
    Planned,

    /// Feature is currently being executed
    InProgress,

    /// Feature execution completed successfully
    Completed,

    /// Feature execution failed
    Failed,
}

/// Git information for worktree-based execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitInfo {
    /// Path to the worktree
    pub worktree_path: String,

    /// Feature branch name
    pub branch: String,

    /// Base branch name
    pub base_branch: String,

    /// Base commit SHA
    pub base_commit: String,
}

/// Execution timing information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionTiming {
    /// Execution start time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,

    /// Execution end time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
}

/// Phase execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseState {
    /// Phase name
    pub name: String,

    /// Phase status
    pub status: PhaseStatus,

    /// When the phase started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,

    /// When the phase completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,

    /// Commit SHA after phase completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,

    /// Summary of phase output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_summary: Option<String>,

    /// Phase execution statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<ExecutionStats>,
}

/// Phase execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum PhaseStatus {
    /// Phase is pending
    #[default]
    Pending,

    /// Phase is in progress
    InProgress,

    /// Phase completed successfully
    Completed,

    /// Phase failed
    Failed,
}

/// Pull request information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestInfo {
    /// PR URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// PR number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,

    /// PR title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// When the PR was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// Whether the PR has been merged
    #[serde(default)]
    pub merged: bool,
}

/// Resume information for interrupted executions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResumeInfo {
    /// Whether the feature can be resumed
    #[serde(default)]
    pub can_resume: bool,

    /// Last completed phase name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_completed_phase: Option<String>,

    /// Next phase to execute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_phase: Option<String>,

    /// When the execution was interrupted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interrupted_at: Option<DateTime<Utc>>,

    /// Reason for interruption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interrupt_reason: Option<InterruptReason>,
}

/// Reason for execution interruption
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InterruptReason {
    /// User cancelled the execution
    UserCancelled,

    /// Execution timed out
    Timeout,

    /// An error occurred
    Error,

    /// System shutdown
    SystemShutdown,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_feature_state_new() {
        let state = FeatureState::new("0001".to_string(), "test-feature".to_string());
        assert_eq!(state.feature.id, "0001");
        assert_eq!(state.feature.slug, "test-feature");
        assert_eq!(state.status, FeatureStatus::Planned);
    }

    #[test]
    fn test_feature_state_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let feature_path = temp_dir.path().join("0001_test-feature");
        std::fs::create_dir_all(&feature_path).unwrap();

        let state = FeatureState::new("0001".to_string(), "test-feature".to_string());
        state.save(&feature_path).unwrap();

        let loaded = FeatureState::load(&feature_path).unwrap();
        assert_eq!(loaded.feature.id, "0001");
        assert_eq!(loaded.feature.slug, "test-feature");
    }

    #[test]
    fn test_update_phase() {
        let mut state = FeatureState::new("0001".to_string(), "test".to_string());

        state.update_phase("build", PhaseStatus::InProgress, None, None);
        assert_eq!(state.phases.len(), 1);
        assert_eq!(state.phases[0].status, PhaseStatus::InProgress);

        let stats = ExecutionStats {
            turns: 5,
            cost_usd: 0.5,
            ..Default::default()
        };
        state.update_phase(
            "build",
            PhaseStatus::Completed,
            Some(&stats),
            Some("Build completed".to_string()),
        );
        assert_eq!(state.phases[0].status, PhaseStatus::Completed);
        assert_eq!(state.total_stats.turns, 5);
    }

    #[test]
    fn test_mark_for_resume() {
        let mut state = FeatureState::new("0001".to_string(), "test".to_string());
        state.phases.push(PhaseState {
            name: "build".to_string(),
            status: PhaseStatus::Completed,
            started_at: None,
            completed_at: None,
            commit_sha: None,
            output_summary: None,
            stats: None,
        });
        state.phases.push(PhaseState {
            name: "test".to_string(),
            status: PhaseStatus::InProgress,
            started_at: None,
            completed_at: None,
            commit_sha: None,
            output_summary: None,
            stats: None,
        });
        state.current_phase = 1;

        state.mark_for_resume(InterruptReason::UserCancelled);

        assert!(state.resume.can_resume);
        assert_eq!(state.resume.last_completed_phase, Some("build".to_string()));
        assert_eq!(state.resume.next_phase, Some("test".to_string()));
        assert_eq!(
            state.resume.interrupt_reason,
            Some(InterruptReason::UserCancelled)
        );
    }

    #[test]
    fn test_next_feature_id() {
        let temp_dir = TempDir::new().unwrap();
        let gba_path = temp_dir.path();

        // No features directory
        let id = FeatureState::next_feature_id(gba_path).unwrap();
        assert_eq!(id, "0001");

        // Create features directory with some features
        let features_path = gba_path.join("features");
        std::fs::create_dir_all(&features_path).unwrap();
        std::fs::create_dir(features_path.join("0001_first")).unwrap();
        std::fs::create_dir(features_path.join("0003_third")).unwrap();

        let id = FeatureState::next_feature_id(gba_path).unwrap();
        assert_eq!(id, "0004");
    }

    #[test]
    fn test_feature_status_serialization() {
        let status = FeatureStatus::InProgress;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"inProgress\"");
    }

    #[test]
    fn test_phase_status_serialization() {
        let status = PhaseStatus::Completed;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"completed\"");
    }
}
