use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for the GBA core engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Repository path to work with
    pub repo_path: PathBuf,
    /// Claude API key
    pub api_key: String,
    /// Model to use (default: claude-sonnet-4-5-20250929)
    pub model: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            repo_path: PathBuf::from("."),
            api_key: String::new(),
            model: "claude-sonnet-4-5-20250929".to_string(),
        }
    }
}

/// Core execution engine for GBA
pub struct Engine {
    config: Config,
}

impl Engine {
    /// Create a new engine instance
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Execute a task with the given prompt
    pub async fn execute(&self, prompt: &str) -> Result<String> {
        // TODO: Implement Claude Agent SDK integration
        Ok(format!("Executing: {}", prompt))
    }

    /// Get the current configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.model, "claude-sonnet-4-5-20250929");
    }

    #[tokio::test]
    async fn test_engine_execute() {
        let config = Config::default();
        let engine = Engine::new(config);
        let result = engine.execute("test prompt").await;
        assert!(result.is_ok());
    }
}
