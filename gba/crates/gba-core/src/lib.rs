//! GBA Core Engine (gba-core)
//!
//! This crate provides the core execution engine for the GBA project.
//! It orchestrates Claude Agent SDK calls and manages feature execution.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use claude_agent_sdk_rs::{
    ClaudeAgentOptions, ClaudeClient, ContentBlock, Message, PermissionMode, SystemPrompt,
    SystemPromptPreset,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{error, info, instrument};

pub mod state;

pub use state::{
    ExecutionTiming, FeatureInfo, FeatureState, FeatureStatus, GitInfo, InterruptReason,
    PhaseState, PhaseStatus, PullRequestInfo, ResumeInfo,
};

/// Errors that can occur in the core engine
#[derive(Error, Debug)]
pub enum CoreError {
    /// Agent execution failed
    #[error("Agent execution failed: {0}")]
    AgentExecutionFailed(String),

    /// Agent timeout
    #[error("Agent timeout after {0:?}")]
    AgentTimeout(Duration),

    /// Invalid execution context
    #[error("Invalid execution context: {0}")]
    InvalidContext(String),

    /// Claude SDK error
    #[error("Claude SDK error: {0}")]
    SdkError(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Phase not found
    #[error("Phase not found: {0}")]
    PhaseNotFound(String),

    /// Feature not found
    #[error("Feature not found: {0}")]
    FeatureNotFound(String),
}

/// Result type for core operations
pub type Result<T> = std::result::Result<T, CoreError>;

/// Configuration for the GBA core engine
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// Repository path to work with
    pub repo_path: PathBuf,

    /// Claude API key
    #[serde(skip_serializing)]
    pub api_key: String,

    /// Model to use (default: claude-sonnet-4-5-20250929)
    pub model: String,

    /// Maximum turns per phase
    #[serde(default = "default_max_turns")]
    pub max_turns: u32,

    /// Timeout per phase in seconds
    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: u64,

    /// Permission mode
    #[serde(default)]
    pub permission_mode: ConfigPermissionMode,
}

fn default_max_turns() -> u32 {
    50
}

fn default_timeout_seconds() -> u64 {
    300
}

/// Permission mode configuration
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ConfigPermissionMode {
    /// Default permission mode
    #[default]
    Default,
    /// Accept edits automatically
    AcceptEdits,
    /// Plan mode
    Plan,
    /// Bypass all permissions
    BypassPermissions,
}

impl From<ConfigPermissionMode> for PermissionMode {
    fn from(mode: ConfigPermissionMode) -> Self {
        match mode {
            ConfigPermissionMode::Default => PermissionMode::Default,
            ConfigPermissionMode::AcceptEdits => PermissionMode::AcceptEdits,
            ConfigPermissionMode::Plan => PermissionMode::Plan,
            ConfigPermissionMode::BypassPermissions => PermissionMode::BypassPermissions,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            repo_path: PathBuf::from("."),
            api_key: String::new(),
            model: "claude-sonnet-4-5-20250929".to_string(),
            max_turns: default_max_turns(),
            timeout_seconds: default_timeout_seconds(),
            permission_mode: ConfigPermissionMode::default(),
        }
    }
}

/// Execution request for a single agent task
#[derive(Debug, Clone)]
pub struct ExecutionRequest {
    /// System prompt (None = use claude_code preset)
    pub system_prompt: Option<String>,

    /// User prompt
    pub user_prompt: String,

    /// Tools to allow (empty = all tools)
    pub tools: Vec<String>,

    /// Tools to disallow
    pub disallowed_tools: Vec<String>,

    /// Execution context
    pub context: ExecutionContext,

    /// Timeout for this execution
    pub timeout: Option<Duration>,
}

/// Execution context
#[derive(Debug, Clone, Default)]
pub struct ExecutionContext {
    /// Repository path
    pub repo_path: PathBuf,

    /// Feature slug
    pub feature_slug: String,

    /// Feature ID
    pub feature_id: String,

    /// Phase name
    pub phase_name: Option<String>,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Execution result
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Whether execution succeeded
    pub success: bool,

    /// Output text
    pub output: String,

    /// Artifacts produced
    pub artifacts: Vec<Artifact>,

    /// Execution duration
    pub duration: Duration,

    /// Execution statistics
    pub stats: ExecutionStats,
}

/// Execution statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionStats {
    /// Number of turns
    pub turns: u32,

    /// Input tokens used
    pub input_tokens: u64,

    /// Output tokens used
    pub output_tokens: u64,

    /// Cost in USD
    pub cost_usd: f64,
}

/// Artifact produced by execution
#[derive(Debug, Clone)]
pub struct Artifact {
    /// File path
    pub path: PathBuf,

    /// Content
    pub content: String,

    /// Artifact type
    pub artifact_type: ArtifactType,
}

/// Artifact type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactType {
    /// Code file
    Code,
    /// Documentation
    Documentation,
    /// Test file
    Test,
    /// Review notes
    Review,
}

/// Phase definition
#[derive(Debug, Clone)]
pub struct Phase {
    /// Phase name
    pub name: String,

    /// Phase description
    pub description: String,

    /// Whether to use claude_code preset
    pub preset: bool,

    /// Tools to allow (empty = all)
    pub tools: Vec<String>,

    /// Tools to disallow
    pub disallowed_tools: Vec<String>,

    /// Execution context
    pub context: ExecutionContext,
}

/// Core execution engine for GBA
pub struct Engine {
    config: Config,
}

impl std::fmt::Debug for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine")
            .field("repo_path", &self.config.repo_path)
            .field("model", &self.config.model)
            .finish()
    }
}

impl Engine {
    /// Create a new engine instance
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Get the current configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Execute a task with the given prompt (simple API)
    #[instrument(skip(self, prompt), fields(prompt_len = prompt.len()))]
    pub async fn execute(&self, prompt: &str) -> Result<String> {
        let request = ExecutionRequest {
            system_prompt: None,
            user_prompt: prompt.to_string(),
            tools: vec![],
            disallowed_tools: vec![],
            context: ExecutionContext {
                repo_path: self.config.repo_path.clone(),
                ..Default::default()
            },
            timeout: Some(Duration::from_secs(self.config.timeout_seconds)),
        };

        let result = self.execute_request(request).await?;
        Ok(result.output)
    }

    /// Execute a full execution request
    #[instrument(skip(self, request), fields(phase = ?request.context.phase_name))]
    pub async fn execute_request(&self, request: ExecutionRequest) -> Result<ExecutionResult> {
        let start = std::time::Instant::now();

        // Build Claude agent options - use the builder pattern correctly
        // The typed-builder requires all optional fields to be set in one chain
        let system_prompt = if let Some(ref sp) = request.system_prompt {
            SystemPrompt::Text(sp.clone())
        } else {
            SystemPrompt::Preset(SystemPromptPreset::new("claude_code"))
        };

        let options = ClaudeAgentOptions::builder()
            .model(&self.config.model)
            .max_turns(self.config.max_turns)
            .permission_mode(self.config.permission_mode.into())
            .cwd(&request.context.repo_path)
            .system_prompt(system_prompt)
            .allowed_tools(request.tools.clone())
            .disallowed_tools(request.disallowed_tools.clone())
            .build();

        // Create client and execute
        let mut client = ClaudeClient::new(options);

        // Connect with timeout
        let timeout = request
            .timeout
            .unwrap_or(Duration::from_secs(self.config.timeout_seconds));

        let connect_result = tokio::time::timeout(Duration::from_secs(30), client.connect()).await;

        match connect_result {
            Ok(Ok(())) => {}
            Ok(Err(e)) => {
                return Err(CoreError::SdkError(format!("Failed to connect: {}", e)));
            }
            Err(_) => {
                return Err(CoreError::AgentTimeout(Duration::from_secs(30)));
            }
        }

        // Send query
        if let Err(e) = client.query(&request.user_prompt).await {
            let _ = client.disconnect().await;
            return Err(CoreError::SdkError(format!("Failed to send query: {}", e)));
        }

        // Collect response with timeout
        let mut full_output = String::new();
        let mut stats = ExecutionStats::default();
        let mut success = true;

        let response_result = tokio::time::timeout(timeout, async {
            let mut stream = client.receive_response();
            while let Some(result) = stream.next().await {
                match result {
                    Ok(Message::Assistant(msg)) => {
                        for block in &msg.message.content {
                            if let ContentBlock::Text(text) = block {
                                full_output.push_str(&text.text);
                            }
                        }
                    }
                    Ok(Message::Result(result_msg)) => {
                        stats.turns = result_msg.num_turns;
                        if let Some(cost) = result_msg.total_cost_usd {
                            stats.cost_usd = cost;
                        }
                        if result_msg.is_error {
                            success = false;
                        }
                        break;
                    }
                    Ok(_) => {
                        // Ignore other message types
                    }
                    Err(e) => {
                        error!("Error receiving message: {}", e);
                        success = false;
                        break;
                    }
                }
            }
        })
        .await;

        // Disconnect
        let _ = client.disconnect().await;

        match response_result {
            Ok(()) => {}
            Err(_) => {
                return Err(CoreError::AgentTimeout(timeout));
            }
        }

        let duration = start.elapsed();

        Ok(ExecutionResult {
            success,
            output: full_output,
            artifacts: vec![],
            duration,
            stats,
        })
    }

    /// Execute multiple phases sequentially
    #[instrument(skip(self, phases), fields(num_phases = phases.len()))]
    pub async fn execute_phases(&self, phases: Vec<Phase>) -> Result<Vec<ExecutionResult>> {
        let mut results = Vec::with_capacity(phases.len());

        for (idx, phase) in phases.into_iter().enumerate() {
            info!("Executing phase {}: {}", idx + 1, phase.name);

            let request = ExecutionRequest {
                system_prompt: if phase.preset {
                    None
                } else {
                    // Load system prompt from template
                    // For now, use None (preset)
                    None
                },
                user_prompt: format!(
                    "Phase: {}\nDescription: {}\n\nExecute this phase.",
                    phase.name, phase.description
                ),
                tools: phase.tools,
                disallowed_tools: phase.disallowed_tools,
                context: phase.context,
                timeout: Some(Duration::from_secs(self.config.timeout_seconds)),
            };

            let result = self.execute_request(request).await?;

            if !result.success {
                error!("Phase {} failed", phase.name);
                return Err(CoreError::AgentExecutionFailed(format!(
                    "Phase {} failed",
                    phase.name
                )));
            }

            results.push(result);
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.model, "claude-sonnet-4-5-20250929");
        assert_eq!(config.max_turns, 50);
        assert_eq!(config.timeout_seconds, 300);
    }

    #[test]
    fn test_execution_context_default() {
        let ctx = ExecutionContext::default();
        assert!(ctx.feature_slug.is_empty());
        assert!(ctx.phase_name.is_none());
    }

    #[test]
    fn test_execution_stats_default() {
        let stats = ExecutionStats::default();
        assert_eq!(stats.turns, 0);
        assert_eq!(stats.cost_usd, 0.0);
    }

    #[test]
    fn test_permission_mode_conversion() {
        assert_eq!(
            PermissionMode::from(ConfigPermissionMode::Default),
            PermissionMode::Default
        );
        assert_eq!(
            PermissionMode::from(ConfigPermissionMode::BypassPermissions),
            PermissionMode::BypassPermissions
        );
    }

    #[tokio::test]
    async fn test_engine_creation() {
        let config = Config::default();
        let engine = Engine::new(config);
        assert_eq!(engine.config().model, "claude-sonnet-4-5-20250929");
    }
}
