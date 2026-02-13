//! GBA Prompt Manager (gba-pm)
//!
//! This crate provides prompt template management for the GBA project.
//! It handles loading, rendering, and caching of prompt templates using minijinja.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use minijinja::{Environment, Value, context};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, instrument, warn};

/// Errors that can occur in the prompt manager
#[derive(Error, Debug)]
pub enum PromptError {
    /// Template not found
    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    /// Template render error
    #[error("Template render error: {0}")]
    RenderError(String),

    /// Template syntax error
    #[error("Template syntax error: {0}")]
    SyntaxError(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// YAML parsing error
    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    /// Invalid feature path
    #[error("Invalid feature path")]
    InvalidFeaturePath,

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// Result type for prompt operations
pub type Result<T> = std::result::Result<T, PromptError>;

/// Task configuration loaded from config.yml
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TaskConfig {
    /// Whether to use claude_code preset (true) or custom system.md (false)
    #[serde(default)]
    pub preset: bool,

    /// List of allowed tools (empty = all tools)
    #[serde(default)]
    pub tools: Vec<String>,

    /// List of disallowed tools
    #[serde(default)]
    pub disallowed_tools: Vec<String>,
}

/// Prompt context for rendering templates
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PromptContext {
    /// Repository path
    pub repo_path: String,

    /// Feature slug (e.g., "user-auth")
    pub feature_slug: String,

    /// Feature ID (e.g., "0001")
    pub feature_id: String,

    /// Current phase name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,

    /// Extra context variables
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl PromptContext {
    /// Create a new prompt context
    pub fn new(repo_path: String, feature_slug: String, feature_id: String) -> Self {
        Self {
            repo_path,
            feature_slug,
            feature_id,
            phase: None,
            extra: HashMap::new(),
        }
    }

    /// Set the current phase
    pub fn with_phase(mut self, phase: impl Into<String>) -> Self {
        self.phase = Some(phase.into());
        self
    }

    /// Add extra context variable
    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }

    /// Generate a cache key for this context
    pub fn cache_key(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.repo_path,
            self.feature_slug,
            self.feature_id,
            self.phase.as_deref().unwrap_or("")
        )
    }
}

/// Resume context for interrupted executions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResumeContext {
    /// Last completed phase name
    pub last_completed_phase: String,

    /// When the execution was interrupted
    pub interrupted_at: String,

    /// Reason for interruption
    pub interrupt_reason: String,

    /// List of completed phases
    pub completed_phases: Vec<String>,
}

/// Internal state for the prompt manager
struct PromptManagerInner {
    /// Template directory path
    template_dir: PathBuf,

    /// Minijinja environment
    env: Environment<'static>,

    /// Rendered template cache
    cache: RwLock<HashMap<String, String>>,
}

/// Prompt manager for handling templates
pub struct PromptManager {
    inner: Arc<PromptManagerInner>,
}

impl std::fmt::Debug for PromptManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PromptManager")
            .field("template_dir", &self.inner.template_dir)
            .finish()
    }
}

impl PromptManager {
    /// Create a new prompt manager with the given template directory
    ///
    /// # Arguments
    ///
    /// * `template_dir` - Path to the directory containing prompt templates
    ///
    /// # Errors
    ///
    /// Returns an error if the template directory doesn't exist
    #[instrument(skip_all, fields(template_dir = %template_dir.display()))]
    pub fn new(template_dir: PathBuf) -> Result<Self> {
        if !template_dir.exists() {
            return Err(PromptError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Template directory not found: {}", template_dir.display()),
            )));
        }

        let mut env = Environment::new();

        // Configure path loader for template discovery
        let template_dir_clone = template_dir.clone();
        env.set_loader(move |name| {
            let path = template_dir_clone.join(name);
            match std::fs::read_to_string(&path) {
                Ok(content) => Ok(Some(content)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(minijinja::Error::new(
                    minijinja::ErrorKind::InvalidOperation,
                    format!("Failed to read template: {e}"),
                )),
            }
        });

        // Add custom filters
        env.add_filter("slugify", slugify_filter);
        env.add_filter("indent", indent_filter);

        // Add custom functions
        env.add_function("read_file", read_file_function);
        env.add_function("list_files", list_files_function);

        debug!(
            "PromptManager initialized with template_dir: {:?}",
            template_dir
        );

        Ok(Self {
            inner: Arc::new(PromptManagerInner {
                template_dir,
                env,
                cache: RwLock::new(HashMap::new()),
            }),
        })
    }

    /// Render a template with the given context
    ///
    /// # Arguments
    ///
    /// * `template_name` - Name of the template file (relative to template_dir)
    /// * `ctx` - Context data for rendering
    ///
    /// # Errors
    ///
    /// Returns an error if the template is not found or rendering fails
    #[instrument(skip(self, ctx), fields(template = %template_name))]
    pub fn render(&self, template_name: &str, ctx: &PromptContext) -> Result<String> {
        // Check cache first
        let cache_key = format!("{}:{}", template_name, ctx.cache_key());
        {
            let cache = self.inner.cache.read();
            if let Some(cached) = cache.get(&cache_key) {
                debug!("Cache hit for template: {}", template_name);
                return Ok(cached.clone());
            }
        }

        // Load and render template
        let template = self
            .inner
            .env
            .get_template(template_name)
            .map_err(|e| PromptError::TemplateNotFound(format!("{}: {}", template_name, e)))?;

        let rendered = template
            .render(context! {
                repo_path => &ctx.repo_path,
                feature_slug => &ctx.feature_slug,
                feature_id => &ctx.feature_id,
                phase => &ctx.phase,
                extra => &ctx.extra,
            })
            .map_err(|e| PromptError::RenderError(e.to_string()))?;

        // Cache result
        {
            let mut cache = self.inner.cache.write();
            cache.insert(cache_key, rendered.clone());
        }

        debug!("Rendered template: {}", template_name);
        Ok(rendered)
    }

    /// Load both system and user prompts for a phase
    ///
    /// # Arguments
    ///
    /// * `phase_name` - Name of the phase (e.g., "build", "test")
    /// * `ctx` - Context data for rendering
    ///
    /// # Returns
    ///
    /// A tuple of (system_prompt, user_prompt)
    ///
    /// # Errors
    ///
    /// Returns an error if either template is not found or rendering fails
    #[instrument(skip(self, ctx), fields(phase = %phase_name))]
    pub fn load_phase_prompts(
        &self,
        phase_name: &str,
        ctx: &PromptContext,
    ) -> Result<(String, String)> {
        let system_path = format!("{}/system.md", phase_name);
        let user_path = format!("{}/user.md", phase_name);

        let system_prompt = self.render(&system_path, ctx)?;
        let user_prompt = self.render(&user_path, ctx)?;

        Ok((system_prompt, user_prompt))
    }

    /// Load task configuration from config.yml
    ///
    /// # Arguments
    ///
    /// * `task_name` - Name of the task (e.g., "build", "test")
    ///
    /// # Errors
    ///
    /// Returns an error if the config file is not found or parsing fails
    #[instrument(skip(self), fields(task = %task_name))]
    pub fn load_task_config(&self, task_name: &str) -> Result<TaskConfig> {
        let config_path = self.inner.template_dir.join(task_name).join("config.yml");

        if !config_path.exists() {
            debug!("No config.yml found for task {}, using defaults", task_name);
            return Ok(TaskConfig::default());
        }

        let content = std::fs::read_to_string(&config_path)?;
        let config: TaskConfig = serde_yaml::from_str(&content)?;

        debug!("Loaded config for task {}: {:?}", task_name, config);
        Ok(config)
    }

    /// List all available templates
    ///
    /// # Returns
    ///
    /// A list of template names (directories containing system.md or user.md)
    #[instrument(skip(self))]
    pub fn list_templates(&self) -> Result<Vec<String>> {
        let mut templates = Vec::new();

        for entry in std::fs::read_dir(&self.inner.template_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir()
                && let Some(name) = path.file_name().and_then(|n| n.to_str())
            {
                // Check if directory contains system.md or user.md
                let has_system = path.join("system.md").exists();
                let has_user = path.join("user.md").exists();

                if has_system || has_user {
                    templates.push(name.to_string());
                }
            }
        }

        templates.sort();
        Ok(templates)
    }

    /// Validate a template
    ///
    /// # Arguments
    ///
    /// * `template_name` - Name of the template file to validate
    ///
    /// # Errors
    ///
    /// Returns an error if the template has syntax errors
    #[instrument(skip(self), fields(template = %template_name))]
    pub fn validate(&self, template_name: &str) -> Result<()> {
        self.inner
            .env
            .get_template(template_name)
            .map_err(|e| PromptError::SyntaxError(e.to_string()))?;

        debug!("Template {} is valid", template_name);
        Ok(())
    }

    /// Clear the template cache
    pub fn clear_cache(&self) {
        let mut cache = self.inner.cache.write();
        cache.clear();
        debug!("Template cache cleared");
    }

    /// Get the template directory path
    pub fn template_dir(&self) -> &Path {
        &self.inner.template_dir
    }
}

// Custom filters

/// Slugify filter - converts text to URL-friendly slug
fn slugify_filter(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Indent filter - adds indentation to each line
fn indent_filter(value: &str, spaces: usize) -> String {
    let indent = " ".repeat(spaces);
    value
        .lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("{}{}", indent, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// Custom functions

/// Read file function - reads content from a file path
fn read_file_function(path: &str) -> std::result::Result<String, minijinja::Error> {
    std::fs::read_to_string(path).map_err(|e| {
        minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("Failed to read file '{}': {}", path, e),
        )
    })
}

/// List files function - lists files matching a glob pattern
fn list_files_function(
    dir: &str,
    pattern: &str,
) -> std::result::Result<Vec<String>, minijinja::Error> {
    let glob_pattern = format!("{}/{}", dir, pattern);
    let paths: Vec<String> = glob::glob(&glob_pattern)
        .map_err(|e| {
            minijinja::Error::new(
                minijinja::ErrorKind::InvalidOperation,
                format!("Invalid glob pattern '{}': {}", glob_pattern, e),
            )
        })?
        .filter_map(|p| p.ok())
        .filter_map(|p| p.to_str().map(String::from))
        .collect();

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_templates(dir: &Path) {
        // Create build task directory
        let build_dir = dir.join("build");
        std::fs::create_dir_all(&build_dir).unwrap();

        std::fs::write(
            build_dir.join("system.md"),
            "You are a Rust developer.\nProject: {{ repo_path }}",
        )
        .unwrap();

        std::fs::write(
            build_dir.join("user.md"),
            "Build feature {{ feature_slug }} (ID: {{ feature_id }})\n{% if phase %}Phase: {{ phase }}{% endif %}",
        )
        .unwrap();

        std::fs::write(
            build_dir.join("config.yml"),
            "preset: false\ntools: []\ndisallowedTools: []",
        )
        .unwrap();

        // Create test task directory
        let test_dir = dir.join("test");
        std::fs::create_dir_all(&test_dir).unwrap();

        std::fs::write(test_dir.join("system.md"), "You are a test engineer.").unwrap();

        std::fs::write(test_dir.join("user.md"), "Test feature {{ feature_slug }}").unwrap();
    }

    #[test]
    fn test_should_create_prompt_manager() {
        let temp_dir = TempDir::new().unwrap();
        create_test_templates(temp_dir.path());

        let pm = PromptManager::new(temp_dir.path().to_path_buf());
        assert!(pm.is_ok());
    }

    #[test]
    fn test_should_fail_with_nonexistent_directory() {
        let result = PromptManager::new(PathBuf::from("/nonexistent/path"));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PromptError::Io(_)));
    }

    #[test]
    fn test_should_render_template() {
        let temp_dir = TempDir::new().unwrap();
        create_test_templates(temp_dir.path());

        let pm = PromptManager::new(temp_dir.path().to_path_buf()).unwrap();
        let ctx = PromptContext::new(
            "/path/to/repo".to_string(),
            "user-auth".to_string(),
            "0001".to_string(),
        );

        let result = pm.render("build/user.md", &ctx);
        assert!(result.is_ok());

        let rendered = result.unwrap();
        assert!(rendered.contains("user-auth"));
        assert!(rendered.contains("0001"));
    }

    #[test]
    fn test_should_render_with_phase() {
        let temp_dir = TempDir::new().unwrap();
        create_test_templates(temp_dir.path());

        let pm = PromptManager::new(temp_dir.path().to_path_buf()).unwrap();
        let ctx = PromptContext::new(
            "/path/to/repo".to_string(),
            "user-auth".to_string(),
            "0001".to_string(),
        )
        .with_phase("build");

        let result = pm.render("build/user.md", &ctx);
        assert!(result.is_ok());

        let rendered = result.unwrap();
        assert!(rendered.contains("Phase: build"));
    }

    #[test]
    fn test_should_load_phase_prompts() {
        let temp_dir = TempDir::new().unwrap();
        create_test_templates(temp_dir.path());

        let pm = PromptManager::new(temp_dir.path().to_path_buf()).unwrap();
        let ctx = PromptContext::new(
            "/path/to/repo".to_string(),
            "user-auth".to_string(),
            "0001".to_string(),
        );

        let result = pm.load_phase_prompts("build", &ctx);
        assert!(result.is_ok());

        let (system, user) = result.unwrap();
        assert!(system.contains("Rust developer"));
        assert!(user.contains("user-auth"));
    }

    #[test]
    fn test_should_load_task_config() {
        let temp_dir = TempDir::new().unwrap();
        create_test_templates(temp_dir.path());

        let pm = PromptManager::new(temp_dir.path().to_path_buf()).unwrap();
        let config = pm.load_task_config("build").unwrap();

        assert!(!config.preset);
        assert!(config.tools.is_empty());
    }

    #[test]
    fn test_should_return_default_config_when_missing() {
        let temp_dir = TempDir::new().unwrap();
        create_test_templates(temp_dir.path());

        let pm = PromptManager::new(temp_dir.path().to_path_buf()).unwrap();
        let config = pm.load_task_config("nonexistent").unwrap();

        assert!(!config.preset);
        assert!(config.tools.is_empty());
    }

    #[test]
    fn test_should_list_templates() {
        let temp_dir = TempDir::new().unwrap();
        create_test_templates(temp_dir.path());

        let pm = PromptManager::new(temp_dir.path().to_path_buf()).unwrap();
        let templates = pm.list_templates().unwrap();

        assert!(templates.contains(&"build".to_string()));
        assert!(templates.contains(&"test".to_string()));
    }

    #[test]
    fn test_should_validate_template() {
        let temp_dir = TempDir::new().unwrap();
        create_test_templates(temp_dir.path());

        let pm = PromptManager::new(temp_dir.path().to_path_buf()).unwrap();
        let result = pm.validate("build/system.md");
        assert!(result.is_ok());
    }

    #[test]
    fn test_should_fail_validation_for_nonexistent_template() {
        let temp_dir = TempDir::new().unwrap();
        create_test_templates(temp_dir.path());

        let pm = PromptManager::new(temp_dir.path().to_path_buf()).unwrap();
        let result = pm.validate("nonexistent.md");
        assert!(result.is_err());
    }

    #[test]
    fn test_should_cache_rendered_templates() {
        let temp_dir = TempDir::new().unwrap();
        create_test_templates(temp_dir.path());

        let pm = PromptManager::new(temp_dir.path().to_path_buf()).unwrap();
        let ctx = PromptContext::new(
            "/path/to/repo".to_string(),
            "user-auth".to_string(),
            "0001".to_string(),
        );

        // First render
        let result1 = pm.render("build/user.md", &ctx).unwrap();

        // Second render (should hit cache)
        let result2 = pm.render("build/user.md", &ctx).unwrap();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_should_clear_cache() {
        let temp_dir = TempDir::new().unwrap();
        create_test_templates(temp_dir.path());

        let pm = PromptManager::new(temp_dir.path().to_path_buf()).unwrap();
        let ctx = PromptContext::new(
            "/path/to/repo".to_string(),
            "user-auth".to_string(),
            "0001".to_string(),
        );

        pm.render("build/user.md", &ctx).unwrap();
        pm.clear_cache();

        // Cache should be empty now
        let cache = pm.inner.cache.read();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_slugify_filter() {
        assert_eq!(slugify_filter("Hello World"), "hello-world");
        assert_eq!(slugify_filter("User Authentication"), "user-authentication");
        assert_eq!(slugify_filter("API v2.0"), "api-v2-0");
        assert_eq!(slugify_filter("  spaces  "), "spaces");
    }

    #[test]
    fn test_indent_filter() {
        assert_eq!(indent_filter("line1\nline2", 2), "  line1\n  line2");
        assert_eq!(indent_filter("single", 4), "    single");
        assert_eq!(indent_filter("", 2), "");
        assert_eq!(indent_filter("a\n\nb", 2), "  a\n\n  b");
    }

    #[test]
    fn test_prompt_context_builder() {
        let ctx = PromptContext::new(
            "/repo".to_string(),
            "feature".to_string(),
            "0001".to_string(),
        )
        .with_phase("build")
        .with_extra("key", "value");

        assert_eq!(ctx.repo_path, "/repo");
        assert_eq!(ctx.feature_slug, "feature");
        assert_eq!(ctx.feature_id, "0001");
        assert_eq!(ctx.phase, Some("build".to_string()));
        assert!(ctx.extra.contains_key("key"));
    }
}
