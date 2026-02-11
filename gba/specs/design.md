# GBA (Geektime Bootcamp Agent) Design Document

## 1. Overview

GBA is a command-line tool that wraps the Claude Agent SDK to help developers add new features to repositories through an AI-assisted workflow. It provides three main commands: `init`, `plan`, and `run`.

### 1.1 Core Concepts

- **Feature-driven development**: Each feature has its own directory with specs, docs, and execution trees
- **Plan-then-execute workflow**: Users plan features interactively, then execute them in phases
- **Prompt-based orchestration**: Uses templated prompts to guide Claude Agent SDK through different phases
- **Phase-based execution**: Features are implemented through multiple phases (observe, build, test, review, PR)

## 2. Architecture

### 2.1 System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                           GBA CLI                                    │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                          │
│  │gba init  │  │gba plan  │  │gba run   │                          │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘                          │
└───────┼─────────────┼─────────────┼────────────────────────────────┘
        │             │             │
        │             │             │
┌───────▼─────────────▼─────────────▼────────────────────────────────┐
│                      CLI Layer (clap / ratatui)                     │
│  - Command parsing                                                  │
│  - Interactive UI                                                   │
│  - Progress display                                                 │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
┌─────────────────────────────▼───────────────────────────────────────┐
│                      Runtime Layer                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │
│  │ Init Runtime │  │ Plan Runtime │  │ Run Runtime  │             │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘             │
│         │                  │                  │                      │
│         └──────────────────┼──────────────────┘                      │
└────────────────────────────┼─────────────────────────────────────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
┌───────▼────────┐  ┌────────▼────────┐  ┌───────▼────────┐
│   gba-core     │  │    gba-pm       │  │  File System   │
│                │  │                 │  │                │
│ - AgentRunner │  │ - PromptLoader  │  │ - .gba/        │
│ - TaskExecutor│  │ - PromptRender  │  │ - specs/       │
│ - PhaseManager│  │ - TemplateCache │  │ - docs/        │
└───────┬────────┘  └────────┬────────┘  │ - trees/       │
        │                    │            └────────────────┘
        │                    │
        └────────────────────┘
                   │
┌──────────────────▼──────────────────┐
│     Claude Agent SDK (0.6)          │
│  - Agent execution                  │
│  - Tool calling                     │
│  - Conversation management          │
└─────────────────┬───────────────────┘
                  │
┌─────────────────▼───────────────────┐
│         Tokio Runtime               │
│  - Async task execution             │
│  - Actor-based concurrency          │
└─────────────────────────────────────┘
```

### 2.2 Data Flow

```
User Command
    │
    ▼
CLI Parser (clap)
    │
    ▼
Command Handler
    │
    ├─── init ──▶ Create .gba structure
    │
    ├─── plan ──▶ Interactive Planning
    │              │
    │              ├─▶ Load prompt template (gba-pm)
    │              ├─▶ Render with context (gba-pm)
    │              ├─▶ Execute agent (gba-core)
    │              ├─▶ Collect user feedback (CLI)
    │              └─▶ Save plan to specs/
    │
    └─── run ───▶ Execute Plan
                   │
                   ├─▶ Load plan from specs/
                   ├─▶ For each phase:
                   │    ├─▶ Load phase prompt (gba-pm)
                   │    ├─▶ Render with context (gba-pm)
                   │    ├─▶ Execute agent (gba-core)
                   │    ├─▶ Save results to trees/
                   │    └─▶ Display progress (CLI)
                   └─▶ Complete execution
```

## 3. Crate Responsibilities

### 3.1 gba-core

**Purpose**: Core execution engine that orchestrates Claude Agent SDK calls.

**Responsibilities**:
- Execute agent tasks with different prompts
- Manage execution phases
- Handle agent lifecycle (start, monitor, stop)
- Provide execution context and state management
- Error handling and recovery

**Public Interface**:

```rust
// Core agent runner
pub struct AgentRunner {
    // Internal fields hidden
}

impl AgentRunner {
    /// Create a new agent runner with API key
    pub async fn new(api_key: String) -> Result<Self>;

    /// Execute a single agent task with prompt
    pub async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResult>;

    /// Execute multiple phases sequentially
    pub async fn execute_phases(&self, phases: Vec<Phase>) -> Result<Vec<ExecutionResult>>;
}

// Execution request
pub struct ExecutionRequest {
    pub system_prompt: Option<String>, // None = use claude_code preset
    pub user_prompt: String,
    pub tools: Vec<String>,            // Empty = all tools
    pub context: ExecutionContext,
    pub timeout: Option<Duration>,
}

// Execution context
pub struct ExecutionContext {
    pub repo_path: PathBuf,
    pub feature_slug: String,
    pub phase_name: Option<String>,
    pub metadata: HashMap<String, String>,
}

// Execution result
pub struct ExecutionResult {
    pub success: bool,
    pub output: String,
    pub artifacts: Vec<Artifact>,
    pub duration: Duration,
    pub stats: ExecutionStats,
}

// Execution statistics
pub struct ExecutionStats {
    pub turns: u32,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cost_usd: f64,
}

// Phase definition
pub struct Phase {
    pub name: String,
    pub description: String,
    pub preset: bool,              // true = use claude_code preset
    pub tools: Vec<String>,        // Empty = all tools
    pub context: ExecutionContext,
}

// Artifact produced by execution
pub struct Artifact {
    pub path: PathBuf,
    pub content: String,
    pub artifact_type: ArtifactType,
}

pub enum ArtifactType {
    Code,
    Documentation,
    Test,
    Review,
}

// Feature state management
pub struct FeatureState {
    pub version: String,
    pub feature: FeatureInfo,
    pub status: FeatureStatus,
    pub current_phase: usize,
    pub git: Option<GitInfo>,
    pub phases: Vec<PhaseState>,
    pub total_stats: ExecutionStats,
    pub execution: ExecutionTiming,
    pub pull_request: Option<PullRequestInfo>,
    pub resume: ResumeInfo,
    pub error: Option<String>,
}

pub struct FeatureInfo {
    pub id: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum FeatureStatus {
    Planned,
    InProgress,
    Completed,
    Failed,
}

pub struct GitInfo {
    pub worktree_path: PathBuf,
    pub branch: String,
    pub base_branch: String,
    pub base_commit: String,
}

pub struct ExecutionTiming {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

pub struct PhaseState {
    pub name: String,
    pub status: PhaseStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub commit_sha: Option<String>,
    pub output_summary: Option<String>,
    pub stats: Option<ExecutionStats>,
}

pub enum PhaseStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

pub struct PullRequestInfo {
    pub url: Option<String>,
    pub number: Option<u32>,
    pub title: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub merged: bool,
}

pub struct ResumeInfo {
    pub can_resume: bool,
    pub last_completed_phase: Option<String>,
    pub next_phase: Option<String>,
    pub interrupted_at: Option<DateTime<Utc>>,
    pub interrupt_reason: Option<InterruptReason>,
}

pub enum InterruptReason {
    UserCancelled,
    Timeout,
    Error,
    SystemShutdown,
}

impl FeatureState {
    /// Load state from state.yml file
    pub fn load(feature_path: &Path) -> Result<Self>;

    /// Save state to state.yml file
    pub fn save(&self, feature_path: &Path) -> Result<()>;

    /// Update phase state with execution result
    pub fn update_phase(&mut self, phase_name: &str, status: PhaseStatus, result: &ExecutionResult);

    /// Mark feature as completed
    pub fn complete(&mut self, pr_info: PullRequestInfo);

    /// Mark for resume after interruption
    pub fn mark_for_resume(&mut self, reason: InterruptReason);

    /// Get next feature ID
    pub fn next_feature_id(gba_path: &Path) -> Result<String>;
}
```

**Internal Architecture**:

The core engine is built on tokio async runtime and uses the actor model for managing agent execution lifecycle.

#### 3.1.1 Claude Agent SDK Integration

**Important**: This project uses `claude-agent-sdk-rs` (v0.6.4+) from https://github.com/tyrchen/claude-agent-sdk-rs via Git dependency.

**Key Features**:
- Bidirectional streaming support
- Multi-turn conversations with session management
- Custom tools via MCP servers
- Hooks system for event interception
- Permission callbacks for tool access control

**System Requirements**:
- Rust 1.90.0+ (2024 edition)
- Tokio async runtime
- Environment variable: `ANTHROPIC_API_KEY`

```rust
use claude_agent_sdk_rs::{
    ClaudeClient, ClaudeAgentOptions, Message, ContentBlock, TextBlock
};
use futures::StreamExt;

// Internal implementation details
struct AgentRunnerInner {
    options: ClaudeAgentOptions,
    runtime: tokio::runtime::Handle,
}

impl AgentRunner {
    pub async fn new(api_key: String) -> Result<Self> {
        // Configure agent options with default settings
        let options = ClaudeAgentOptions::builder()
            .api_key(api_key)
            .model("claude-sonnet-4-5")
            .max_turns(10)
            .build()?;

        Ok(Self {
            inner: Arc::new(AgentRunnerInner {
                options,
                runtime: tokio::runtime::Handle::current(),
            }),
        })
    }

    pub async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResult> {
        // Build options based on phase configuration
        let mut options = self.inner.options.clone();

        // Set system prompt
        options.system_prompt = if let Some(custom_system) = request.system_prompt {
            // Use custom system prompt
            Some(SystemPrompt::Text(custom_system))
        } else {
            // Use claude_code preset
            Some(SystemPrompt::Preset(
                SystemPromptPreset::new("claude_code")
            ))
        };

        // Set tools (empty vec = all tools)
        if !request.tools.is_empty() {
            options.tools = request.tools;
        }

        // Create client for this execution
        let mut client = ClaudeClient::new(options)?;

        // Connect to Claude
        client.connect().await?;

        // Send the user prompt
        client.query(&request.user_prompt).await?;

        // Collect responses
        let mut full_output = String::new();
        let mut artifacts = Vec::new();
        let start = std::time::Instant::now();

        // Process streaming messages
        let mut stream = client.receive_response();
        while let Some(message) = stream.next().await {
            match message? {
                Message::Assistant(content) => {
                    // Extract text content from blocks
                    for block in content {
                        if let ContentBlock::Text(TextBlock { text }) = block {
                            full_output.push_str(&text);
                        }
                    }
                }
                Message::Result(_) => {
                    // Conversation completed
                    break;
                }
                _ => {
                    // Handle other message types if needed
                }
            }
        }

        // Disconnect the client
        client.disconnect().await?;

        let duration = start.elapsed();

        Ok(ExecutionResult {
            success: true,
            output: full_output,
            artifacts,
            duration,
        })
    }

    // Execute with timeout
    pub async fn execute_with_timeout(
        &self,
        request: ExecutionRequest,
    ) -> Result<ExecutionResult> {
        match request.timeout {
            Some(timeout) => {
                tokio::time::timeout(timeout, self.execute(request))
                    .await
                    .map_err(|_| CoreError::AgentTimeout(timeout))?
            }
            None => self.execute(request).await,
        }
    }
}
```

#### 3.1.2 Actor-Based Execution Model

The execution engine uses tokio actors for managing concurrent agent executions:

```rust
use tokio::sync::{mpsc, oneshot};
use std::sync::atomic::{AtomicBool, Ordering};

// Actor message types
enum ActorMessage {
    Execute {
        request: ExecutionRequest,
        response_tx: oneshot::Sender<Result<ExecutionResult>>,
    },
    Shutdown,
}

// Agent executor actor
struct AgentExecutorActor {
    receiver: mpsc::Receiver<ActorMessage>,
    agent_runner: AgentRunner,
    shutdown: Arc<AtomicBool>,
}

impl AgentExecutorActor {
    async fn run(mut self) {
        while let Some(msg) = self.receiver.recv().await {
            if self.shutdown.load(Ordering::Relaxed) {
                break;
            }

            match msg {
                ActorMessage::Execute { request, response_tx } => {
                    let result = self.agent_runner.execute(request).await;
                    let _ = response_tx.send(result);
                }
                ActorMessage::Shutdown => {
                    self.shutdown.store(true, Ordering::Relaxed);
                    break;
                }
            }
        }
    }
}

// Actor handle for communication
pub struct AgentExecutorHandle {
    sender: mpsc::Sender<ActorMessage>,
    shutdown: Arc<AtomicBool>,
}

impl AgentExecutorHandle {
    pub fn new(agent_runner: AgentRunner) -> Self {
        let (tx, rx) = mpsc::channel(32);
        let shutdown = Arc::new(AtomicBool::new(false));

        let actor = AgentExecutorActor {
            receiver: rx,
            agent_runner,
            shutdown: shutdown.clone(),
        };

        tokio::spawn(async move {
            actor.run().await;
        });

        Self {
            sender: tx,
            shutdown,
        }
    }

    pub async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResult> {
        let (response_tx, response_rx) = oneshot::channel();

        self.sender
            .send(ActorMessage::Execute { request, response_tx })
            .await
            .map_err(|_| CoreError::AgentExecutionFailed("Actor channel closed".into()))?;

        response_rx
            .await
            .map_err(|_| CoreError::AgentExecutionFailed("Response channel closed".into()))?
    }

    pub async fn shutdown(&self) {
        self.shutdown.store(true, Ordering::Relaxed);
        let _ = self.sender.send(ActorMessage::Shutdown).await;
    }
}
```

#### 3.1.3 Phase Execution with Tokio

Multi-phase execution uses tokio for sequential async operations:

```rust
impl AgentRunner {
    pub async fn execute_phases(&self, phases: Vec<Phase>) -> Result<Vec<ExecutionResult>> {
        let mut results = Vec::with_capacity(phases.len());
        let executor = AgentExecutorHandle::new(self.clone());

        for (idx, phase) in phases.into_iter().enumerate() {
            tracing::info!("Executing phase {}: {}", idx + 1, phase.name);

            // Load prompts based on phase configuration
            let (system_prompt, user_prompt) = if phase.preset {
                // Use claude_code preset, optionally append system.md
                let user_prompt = self.prompt_manager.render(
                    &format!("{}/user.md", phase.name),
                    &phase.context
                )?;

                // Optional: load system.md for append text
                let append_text = self.prompt_manager.render(
                    &format!("{}/system.md", phase.name),
                    &phase.context
                ).ok(); // Ignore if not exists

                (None, user_prompt) // None means use preset
            } else {
                // Use custom system prompt
                let (sys, usr) = self.prompt_manager.load_phase_prompts(
                    &phase.name,
                    &phase.context
                )?;
                (Some(sys), usr)
            };

            // Build request with previous results as context
            let mut context = phase.context;
            if let Some(prev_result) = results.last() {
                context.metadata.insert(
                    "previous_output".to_string(),
                    prev_result.output.clone(),
                );
            }

            let request = ExecutionRequest {
                system_prompt,
                user_prompt,
                tools: phase.tools, // Vec<String>, empty = all tools
                context,
                timeout: Some(Duration::from_secs(300)),
            };

            // Execute phase
            let result = executor.execute(request).await?;

            // Check if phase succeeded
            if !result.success {
                tracing::error!("Phase {} failed: {}", phase.name, result.output);
                return Err(CoreError::AgentExecutionFailed(
                    format!("Phase {} failed", phase.name)
                ));
            }

            results.push(result);
        }

        executor.shutdown().await;
        Ok(results)
    }
}
```

#### 3.1.4 Streaming Output Support

For real-time progress updates, support streaming responses:

```rust
use tokio::sync::broadcast;
use futures::StreamExt;

pub struct StreamingExecutionRequest {
    pub prompt: String,
    pub context: ExecutionContext,
    pub output_tx: broadcast::Sender<String>,
}

impl AgentRunner {
    pub async fn execute_streaming(
        &self,
        request: StreamingExecutionRequest,
    ) -> Result<ExecutionResult> {
        let mut client = ClaudeClient::new(self.inner.options.clone())?;
        client.connect().await?;
        client.query(&request.prompt).await?;

        let mut full_output = String::new();
        let start = std::time::Instant::now();

        // Stream responses to subscribers
        let mut stream = client.receive_response();
        while let Some(message) = stream.next().await {
            match message? {
                Message::Assistant(content) => {
                    for block in content {
                        if let ContentBlock::Text(TextBlock { text }) = block {
                            full_output.push_str(&text);
                            // Broadcast chunk to subscribers
                            let _ = request.output_tx.send(text);
                        }
                    }
                }
                Message::Result(_) => break,
                _ => {}
            }
        }

        client.disconnect().await?;

        Ok(ExecutionResult {
            success: true,
            output: full_output,
            artifacts: vec![],
            duration: start.elapsed(),
        })
    }
}
```

#### 3.1.5 Error Handling and Retry Logic

Implement retry logic with exponential backoff for transient failures:

```rust
use tokio::time::{sleep, Duration};
use claude_agent_sdk_rs::Error as ClaudeError;

async fn execute_with_retry(
    options: &ClaudeAgentOptions,
    prompt: &str,
    max_retries: u32,
) -> Result<String> {
    let mut retries = 0;
    let mut backoff = Duration::from_secs(1);

    loop {
        let mut client = match ClaudeClient::new(options.clone()) {
            Ok(c) => c,
            Err(e) if retries < max_retries && is_retryable(&e) => {
                tracing::warn!(
                    "Client creation failed (attempt {}/{}): {}. Retrying in {:?}",
                    retries + 1,
                    max_retries,
                    e,
                    backoff
                );
                sleep(backoff).await;
                retries += 1;
                backoff *= 2;
                continue;
            }
            Err(e) => return Err(CoreError::SdkError(e)),
        };

        match client.connect().await {
            Ok(_) => {}
            Err(e) if retries < max_retries && is_retryable(&e) => {
                tracing::warn!(
                    "Connection failed (attempt {}/{}): {}. Retrying in {:?}",
                    retries + 1,
                    max_retries,
                    e,
                    backoff
                );
                sleep(backoff).await;
                retries += 1;
                backoff *= 2;
                continue;
            }
            Err(e) => return Err(CoreError::SdkError(e)),
        }

        match client.query(prompt).await {
            Ok(_) => {
                let mut output = String::new();
                let mut stream = client.receive_response();

                while let Some(message) = stream.next().await {
                    if let Ok(Message::Assistant(content)) = message {
                        for block in content {
                            if let ContentBlock::Text(TextBlock { text }) = block {
                                output.push_str(&text);
                            }
                        }
                    } else if let Ok(Message::Result(_)) = message {
                        break;
                    }
                }

                let _ = client.disconnect().await;
                return Ok(output);
            }
            Err(e) if retries < max_retries && is_retryable(&e) => {
                tracing::warn!(
                    "Query failed (attempt {}/{}): {}. Retrying in {:?}",
                    retries + 1,
                    max_retries,
                    e,
                    backoff
                );
                let _ = client.disconnect().await;
                sleep(backoff).await;
                retries += 1;
                backoff *= 2;
            }
            Err(e) => {
                let _ = client.disconnect().await;
                return Err(CoreError::SdkError(e));
            }
        }
    }
}

fn is_retryable(error: &ClaudeError) -> bool {
    // Retry on network errors, rate limits, or timeouts
    matches!(
        error,
        ClaudeError::Network(_)
            | ClaudeError::RateLimit
            | ClaudeError::Timeout
    )
}
```

**Key Design Principles**:
- Use actor model for agent lifecycle management
- Separate actor for each agent execution to isolate failures
- Use channels (mpsc) for communication between CLI and agent
- Implement proper shutdown handling with AtomicBool
- Support both blocking and streaming execution modes
- Retry transient failures with exponential backoff
- Use tokio::time::timeout for execution timeouts
- Leverage tokio's async runtime for all I/O operations

### 3.2 gba-pm (Prompt Manager)

**Purpose**: Manage, load, and render prompt templates using minijinja.

**Responsibilities**:
- Load prompt templates from filesystem
- Render templates with context data
- Cache compiled templates
- Validate template syntax
- Provide template discovery

**Public Interface**:

```rust
// Prompt manager
pub struct PromptManager {
    // Internal fields hidden
}

impl PromptManager {
    /// Create a new prompt manager with template directory
    pub fn new(template_dir: PathBuf) -> Result<Self>;

    /// Load and render a prompt template
    pub fn render(&self, template_name: &str, context: &PromptContext) -> Result<String>;

    /// List available templates
    pub fn list_templates(&self) -> Result<Vec<String>>;

    /// Validate a template
    pub fn validate(&self, template_name: &str) -> Result<()>;
}

// Prompt context for rendering
#[derive(Debug, Clone, serde::Serialize)]
pub struct PromptContext {
    pub repo_path: String,
    pub feature_slug: String,
    pub feature_id: String,
    pub phase: Option<String>,
    // Note: Following "convention over configuration" principle
    // Most context is loaded on-demand by AI reading files directly
    // Only essential variables are pre-loaded
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ResumeContext {
    pub last_completed_phase: String,
    pub interrupted_at: String,
    pub interrupt_reason: String,
    pub completed_phases: Vec<String>,
}

impl PromptContext {
    pub fn new(repo_path: String, feature_slug: String, feature_id: String) -> Self;

    /// Load minimal context - AI reads files as needed
    /// This follows "convention over configuration" principle from design_by_tyrchen.md
    pub fn load(feature_path: &Path, repo_path: &Path) -> Result<Self>;

    /// Add resume information when resuming execution
    pub fn with_resume_info(mut self, resume: ResumeContext) -> Self;

    /// Add extra context variables
    pub fn with_extra(mut self, key: String, value: Value) -> Self;
}
```

**Internal Architecture**:

The prompt manager is built on minijinja template engine with caching and validation.

#### 3.2.1 Minijinja Integration

```rust
use minijinja::{Environment, context, Value};
use std::sync::Arc;
use parking_lot::RwLock;

// Internal implementation
struct PromptManagerInner {
    env: Environment<'static>,
    template_dir: PathBuf,
    cache: Arc<RwLock<HashMap<String, String>>>,
}

impl PromptManager {
    pub fn new(template_dir: PathBuf) -> Result<Self> {
        let mut env = Environment::new();

        // Configure environment
        env.set_loader(minijinja::path_loader(&template_dir));

        // Add custom filters
        env.add_filter("slugify", slugify_filter);
        env.add_filter("indent", indent_filter);

        // Add custom functions
        env.add_function("read_file", read_file_function);
        env.add_function("list_files", list_files_function);

        Ok(Self {
            inner: Arc::new(PromptManagerInner {
                env,
                template_dir,
                cache: Arc::new(RwLock::new(HashMap::new())),
            }),
        })
    }

    pub fn render(&self, template_name: &str, context: &PromptContext) -> Result<String> {
        // Check cache first
        let cache_key = format!("{}:{}", template_name, context.cache_key());
        {
            let cache = self.inner.cache.read();
            if let Some(cached) = cache.get(&cache_key) {
                return Ok(cached.clone());
            }
        }

        // Load and render template
        let template = self.inner.env
            .get_template(template_name)
            .map_err(|e| PromptError::TemplateNotFound(e.to_string()))?;

        let rendered = template
            .render(context!(
                repo_path => &context.repo_path,
                feature_slug => &context.feature_slug,
                phase => &context.phase,
                specs => &context.specs,
                previous_output => &context.previous_output,
                extra => &context.extra,
            ))
            .map_err(|e| PromptError::RenderError(e.to_string()))?;

        // Cache result
        {
            let mut cache = self.inner.cache.write();
            cache.insert(cache_key, rendered.clone());
        }

        Ok(rendered)
    }

    /// Load both system and user prompts for a phase
    /// Returns (system_prompt, user_prompt)
    pub fn load_phase_prompts(
        &self,
        phase_name: &str,
        context: &PromptContext,
    ) -> Result<(String, String)> {
        let system_path = format!("{}/system.md", phase_name);
        let user_path = format!("{}/user.md", phase_name);

        let system_prompt = self.render(&system_path, context)?;
        let user_prompt = self.render(&user_path, context)?;

        Ok((system_prompt, user_prompt))
    }

    pub fn list_templates(&self) -> Result<Vec<String>> {
        let mut templates = Vec::new();

        for entry in std::fs::read_dir(&self.inner.template_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    templates.push(name.to_string());
                }
            }
        }

        Ok(templates)
    }

    pub fn validate(&self, template_name: &str) -> Result<()> {
        self.inner.env
            .get_template(template_name)
            .map_err(|e| PromptError::SyntaxError(e.to_string()))?;

        Ok(())
    }
}

// Custom filters
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

fn indent_filter(value: &str, spaces: usize) -> String {
    let indent = " ".repeat(spaces);
    value
        .lines()
        .map(|line| format!("{}{}", indent, line))
        .collect::<Vec<_>>()
        .join("\n")
}

// Custom functions
fn read_file_function(path: &str) -> Result<String, minijinja::Error> {
    std::fs::read_to_string(path)
        .map_err(|e| minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("Failed to read file: {}", e)
        ))
}

fn list_files_function(dir: &str, pattern: &str) -> Result<Vec<String>, minijinja::Error> {
    let glob_pattern = format!("{}/{}", dir, pattern);
    let paths: Vec<String> = glob::glob(&glob_pattern)
        .map_err(|e| minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("Invalid glob pattern: {}", e)
        ))?
        .filter_map(|p| p.ok())
        .filter_map(|p| p.to_str().map(String::from))
        .collect();

    Ok(paths)
}
```

#### 3.2.2 Template Context Loading

The `PromptContext` follows "convention over configuration" - it automatically loads all required context from standard locations:

```rust
impl PromptContext {
    /// Load complete context from feature directory and repository
    pub fn load(feature_path: &Path, repo_path: &Path) -> Result<Self> {
        let feature_slug = feature_path.file_name()
            .and_then(|n| n.to_str())
            .ok_or(PromptError::InvalidFeaturePath)?
            .to_string();

        // Load from standard locations
        let specs = Self::read_file_or_empty(feature_path.join("specs/design.md"))?;
        let verification_criteria = Self::read_file_or_empty(
            feature_path.join("specs/verification.md")
        )?;
        let coding_standards = Self::read_file_or_empty(repo_path.join("CLAUDE.md"))?;
        let readme = Self::read_file_or_empty(repo_path.join("README.md"))?;

        Ok(Self {
            repo_path: repo_path.to_string_lossy().to_string(),
            feature_slug,
            phase: None,
            specs,
            verification_criteria,
            previous_output: String::new(),
            coding_standards,
            readme,
            resume_info: None,
        })
    }

    /// Read file content or return empty string if file doesn't exist
    fn read_file_or_empty(path: PathBuf) -> Result<String> {
        match std::fs::read_to_string(&path) {
            Ok(content) => Ok(content),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
            Err(e) => Err(PromptError::Io(e)),
        }
    }

    /// Add resume information when resuming execution
    pub fn with_resume_info(mut self, resume: ResumeContext) -> Self {
        self.resume_info = Some(resume);
        self
    }

    /// Set previous phase output
    pub fn with_previous_output(mut self, output: String) -> Self {
        self.previous_output = output;
        self
    }

    /// Set current phase name
    pub fn with_phase(mut self, phase: String) -> Self {
        self.phase = Some(phase);
        self
    }
}
```

**Convention over Configuration**:
- Minimal template variables: `repo_path`, `feature_slug`, `feature_id`
- AI reads files on-demand using Read tool (not pre-loaded)
- Standard file locations (following design_by_tyrchen.md):
  - Design spec: `.gba/{feature_id}_{feature_slug}/specs/design.md`
  - Verification: `.gba/{feature_id}_{feature_slug}/specs/verification.md`
  - Coding standards: `CLAUDE.md`
  - Repository docs: `.gba.md`
  - State: `.gba/{feature_id}_{feature_slug}/state.yml`
- Templates guide AI on what to read and where to find it
- No optional fields or complex conditionals
- Extra context passed via `extra` HashMap when needed

#### 3.2.3 Template Examples

**Simplified Template Approach** (following design_by_tyrchen.md):

Templates use minimal variables. AI reads files directly using Read tool when needed.

**Plan Template** (`prompts/plan/user.md`):
```jinja2
# Feature Planning: {{ feature_slug }}

Repository: {{ repo_path }}
Feature ID: {{ feature_id }}

## Your Task

You are helping plan a new feature. Please:

1. Ask clarifying questions about:
   - Feature requirements and scope
   - Technical approach and architecture
   - Testing strategy
   - Documentation needs

2. Read relevant files to understand the codebase:
   - Use Read tool to read README.md, CLAUDE.md, .gba.md
   - Use Glob/Grep to explore the codebase structure
   - Understand existing patterns and conventions

3. Create a detailed plan:
   - Write specs/design.md with architecture and implementation details
   - Write specs/verification.md with test criteria
   - Identify files to create/modify

4. When plan is approved, write the spec files to:
   - .gba/{{ feature_id }}_{{ feature_slug }}/specs/design.md
   - .gba/{{ feature_id }}_{{ feature_slug }}/specs/verification.md

{% if extra.resume_info %}
## Resume Information
This is a resumed session.
Last completed phase: {{ extra.resume_info.last_completed_phase }}
Interrupted at: {{ extra.resume_info.interrupted_at }}
Reason: {{ extra.resume_info.interrupt_reason }}
{% endif %}

Please start by asking questions to understand the feature requirements.
```

**Build Template** (`prompts/build/user.md`):
```jinja2
# Phase: Build Implementation

Repository: {{ repo_path }}
Feature: {{ feature_slug }} (ID: {{ feature_id }})

## Your Task

Implement the feature according to the design specification.

### Steps

1. Read the design specification:
   - .gba/{{ feature_id }}_{{ feature_slug }}/specs/design.md

2. Read project standards:
   - CLAUDE.md (coding standards)
   - .gba.md (repository structure)

3. Implement the feature:
   - Follow the design document
   - Write clean, idiomatic Rust code
   - Add appropriate error handling
   - Include inline documentation
   - Follow project coding standards

4. Commit your changes with descriptive messages

{% if extra.previous_output %}
## Previous Phase Output
{{ extra.previous_output }}
{% endif %}

{% if extra.resume_info %}
## Resume Information
Resuming from interruption.
Last completed phase: {{ extra.resume_info.last_completed_phase }}
{% endif %}

Begin implementation now.
```

**Key Principles**:
- Minimal template variables (repo_path, feature_slug, feature_id)
- AI reads files using Read tool when needed
- No auto-loading of specs, readme, standards
- Templates guide AI on what to read and where
- Follows "convention over configuration" principle
{% for file in extra.files_to_modify %}
- {{ file }}
{% endfor %}
{% endif %}

Please implement the feature now.
```

**Template Structure**:
```
.gba/
├── prompts/
│   ├── init.md
│   ├── plan.md
│   ├── phase_1_observe.md
│   ├── phase_2_build.md
│   ├── phase_3_test.md
│   ├── phase_4_review.md
│   └── phase_5_pr.md
```

**Key Design Principles**:
- Use minijinja's path loader for automatic template discovery
- Cache rendered templates to avoid re-rendering
- Provide custom filters for common transformations (slugify, indent)
- Provide custom functions for file operations (read_file, list_files)
- Use parking_lot::RwLock for efficient concurrent cache access
- Validate templates on load to fail fast
- Support template inheritance and includes for reusability

#### 3.2.4 Template Organization and System Prompts

**Updated Template Structure** (2026-02-11):

Templates are organized at project root level with per-task configuration:

```
gba/
├── prompts/              # Project-level prompt templates
│   ├── init/
│   │   ├── config.yml   # Task-specific configuration
│   │   ├── system.md    # System prompt (role definition)
│   │   └── user.md      # User prompt (task description)
│   ├── plan/
│   │   ├── config.yml
│   │   ├── system.md
│   │   └── user.md
│   ├── observe/
│   │   ├── config.yml
│   │   ├── system.md
│   │   └── user.md
│   ├── build/
│   │   ├── config.yml
│   │   ├── system.md
│   │   └── user.md
│   ├── test/
│   │   ├── config.yml
│   │   ├── system.md
│   │   └── user.md
│   ├── verification/
│   │   ├── config.yml
│   │   ├── system.md
│   │   └── user.md
│   ├── review/
│   │   ├── config.yml
│   │   ├── system.md
│   │   └── user.md
│   └── pr/
│       ├── config.yml
│       ├── system.md
│       └── user.md
├── crates/
│   └── gba-pm/          # Prompt manager code
└── .gba/                # Per-feature workspace
    └── feature-name/
        └── state.yml    # Feature execution state
```

**Task Configuration** (`config.yml`):

Each task directory contains a `config.yml` that defines task-specific settings:

```yaml
# prompts/build/config.yml
preset: false           # true: use claude_code preset, false: use custom system.md
tools: []              # Empty = all tools, or specify: ["Bash", "Read", "Write"]
disallowedTools: []    # Tools to explicitly disallow (empty = no restrictions)
```

**Examples**:

```yaml
# prompts/build/config.yml - Full development environment
preset: false           # Use custom Rust developer role
tools: []              # All tools available
disallowedTools: []

# prompts/pr/config.yml - Git operations only
preset: true            # Use claude_code preset
tools: ["Bash"]        # Only Bash for git commands
disallowedTools: []

# prompts/observe/config.yml - Read-only exploration
preset: false           # Use custom analyst role
tools: ["Read", "Glob", "Grep"]  # Only read operations
disallowedTools: ["Write", "Edit", "Bash"]
```

**System Prompt vs User Prompt**:

- **System Prompt** (`system.md`): Defines the AI agent's role, expertise, and behavior
  - Role definition (e.g., "You are an expert software architect")
  - Expertise areas and specializations
  - Working principles and standards
  - Tool usage guidelines
  - Code quality and security standards

- **User Prompt** (`user.md`): Defines the specific task and context
  - Task description and objectives
  - Context information (repoPath, specs, etc.)
  - Execution steps and output requirements
  - No role definitions (handled by system prompt)

**Specialized Roles by Phase**:

1. **init, observe**: Base role (general software engineering)
2. **plan**: Architect role (software architecture design)
3. **build**: Developer role (Rust programming)
4. **test**: Tester role (test design and QA)
5. **verification**: QA role (requirements verification)
6. **review**: Reviewer role (code review)
7. **pr**: DevOps role (Git workflows and PR management)

**Preset vs Custom System Prompts**:

Each phase can use either Claude Code preset or custom system prompt:

```rust
use claude_agent_sdk_rs::{ClaudeAgentOptions, SystemPrompt, SystemPromptPreset};

// Option 1: Use Claude Code preset (when config.preset = true)
let options = ClaudeAgentOptions {
    system_prompt: Some(SystemPrompt::Preset(
        SystemPromptPreset::new("claude_code")
    )),
    ..Default::default()
};

// Option 2: Use custom system prompt (when config.preset = false)
let (system_prompt, user_prompt) = prompt_manager.load_phase_prompts("build", &context)?;
let options = ClaudeAgentOptions {
    system_prompt: Some(SystemPrompt::Text(system_prompt)),
    ..Default::default()
};
```

**Convention Over Configuration**:

- Each task is self-contained with its own `config.yml`
- Templates automatically loaded from `prompts/{taskName}/system.md` and `user.md`
- Task configuration loaded from `prompts/{taskName}/config.yml`
- No need to duplicate task settings in project-level config
- Project-level `.gba/config.yml` only contains global settings (API key, model, git settings)

**Benefits**:
- Clear separation of concerns (role vs task)
- Self-contained tasks (config + templates together)
- Simple preset decision (boolean flag per task)
- Flexible tool restrictions per task
- Reusable system prompts across similar tasks
- Better maintainability and flexibility
- Follows prompt engineering best practices
- Fine-grained control over AI behavior and tool access per phase

### 3.3 gba-cli

**Purpose**: Command-line interface for user interaction.

**Responsibilities**:
- Parse command-line arguments
- Provide interactive UI for planning
- Display execution progress
- Handle user input and feedback
- Coordinate between gba-core and gba-pm

**Command Structure**:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gba")]
#[command(about = "Geektime Bootcamp Agent - AI-assisted feature development")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize GBA in current repository
    Init {
        /// Force reinitialize even if .gba exists
        #[arg(short, long)]
        force: bool,
    },

    /// Plan a new feature interactively
    Plan {
        /// Feature slug (e.g., "user-auth", "api-v2")
        feature_slug: String,

        /// Initial feature description
        #[arg(short, long)]
        description: Option<String>,
    },

    /// Execute a planned feature
    Run {
        /// Feature slug to execute
        feature_slug: String,

        /// Resume from specific phase
        #[arg(short, long)]
        resume_from: Option<String>,

        /// Dry run (show what would be executed)
        #[arg(short, long)]
        dry_run: bool,
    },
}
```

**UI Components**:
- Progress bars for phase execution (ratatui)
- Interactive chat for planning (ratatui)
- Colored output for status messages (crossterm)
- Spinner for long-running operations

## 4. File System Structure

### 4.1 Repository Structure After Init

```
<repo>/
├── .gba/
│   ├── config.yaml              # GBA configuration
│   ├── prompts/                 # Prompt templates
│   │   ├── init.md
│   │   ├── plan.md
│   │   ├── phase_1_observe.md
│   │   ├── phase_2_build.md
│   │   ├── phase_3_test.md
│   │   ├── phase_4_verification.md
│   │   ├── phase_5_review.md
│   │   └── phase_6_pr.md
│   └── features/                # Feature-specific data
│       ├── 0001_<feature-slug>/ # Feature with sequential ID
│       │   ├── specs/
│       │   │   ├── design.md
│       │   │   └── verification.md
│       │   ├── docs/
│       │   │   └── impl_details.md
│       │   ├── state.yml        # Execution state and statistics
│       │   └── trees/           # Execution trees (gitignored)
│       └── 0002_<feature-slug>/
├── .trees/                      # Git worktrees (gitignored)
│   ├── 0001_<feature-slug>/     # Isolated working directory
│   └── 0002_<feature-slug>/
├── .gba.md                      # Repository documentation (auto-generated)
└── .gitignore                   # Updated to ignore .trees/ and trees/
```

### 4.2 Configuration File

```yaml
# .gba/config.yaml
version: "0.1.0"

# Agent configuration
agent:
  # API key environment variable name
  apiKeyEnv: "ANTHROPIC_API_KEY"

  # Default Claude model
  model: "claude-sonnet-4-5"

  # Permission mode: auto | manual | none
  permissionMode: "auto"

  # Budget limit in USD (optional)
  budgetLimit: null

  # Timeout in seconds
  timeoutSeconds: 300

# Prompt configuration
prompts:
  # Additional prompt directories (optional)
  include:
    - ~/.config/gba/prompts

# Git configuration
git:
  # Auto-commit after each phase
  autoCommit: true

  # Branch naming pattern
  # Available variables: {id}, {slug}
  branchPattern: "feature/{id}-{slug}"

  # Use git worktree for isolation
  useWorktree: true

  # Base branch for new features
  baseBranch: "main"

# Code review configuration
review:
  # Enable code review phase
  enabled: true

  # Review provider: codex | claude | none
  provider: "codex"

# Phase execution order
# Each phase's configuration is defined in prompts/{phaseName}/config.yml
# This list only defines the execution order and descriptions
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
```

### 4.3 Feature State File

Each feature maintains a `state.yml` file to track execution progress, costs, and results:

```yaml
# .gba/features/0001_user-authentication/state.yml
version: "0.1.0"

# Feature identification
feature:
  id: "0001"
  slug: "user-authentication"
  createdAt: "2026-02-10T10:00:00Z"
  updatedAt: "2026-02-10T12:30:00Z"

# Execution status: planned | in_progress | completed | failed
status: "in_progress"

# Current phase index (0-based, for precise resume)
currentPhase: 2

# Git information (if using worktree)
git:
  worktreePath: ".trees/0001_user-authentication"
  branch: "feature/0001-user-authentication"
  baseBranch: "main"
  baseCommit: "abc1234"

# Phase execution history
phases:
  - name: "observe"
    status: "completed"
    startedAt: "2026-02-10T10:00:00Z"
    completedAt: "2026-02-10T10:15:00Z"
    commitSha: "def5678"
    outputSummary: "Analyzed authentication patterns in codebase"
    stats:
      turns: 8
      inputTokens: 12500
      outputTokens: 8300
      costUsd: 0.42

  - name: "build"
    status: "completed"
    startedAt: "2026-02-10T10:15:00Z"
    completedAt: "2026-02-10T11:00:00Z"
    commitSha: "ghi9012"
    outputSummary: "Implemented JWT-based authentication"
    stats:
      turns: 15
      inputTokens: 45000
      outputTokens: 32000
      costUsd: 0.89

  - name: "test"
    status: "completed"
    startedAt: "2026-02-10T11:00:00Z"
    completedAt: "2026-02-10T11:30:00Z"
    commitSha: "jkl3456"
    outputSummary: "Added unit and integration tests"
    stats:
      turns: 12
      inputTokens: 28000
      outputTokens: 19000
      costUsd: 0.67

  - name: "verification"
    status: "in_progress"
    startedAt: "2026-02-10T11:30:00Z"
    completedAt: null
    commitSha: null
    outputSummary: null
    stats:
      turns: 10
      inputTokens: 15000
      outputTokens: 10000
      costUsd: 0.37

  - name: "review"
    status: "pending"
    startedAt: null
    completedAt: null
    commitSha: null
    outputSummary: null
    stats: null

  - name: "pr"
    status: "pending"
    startedAt: null
    completedAt: null
    commitSha: null
    outputSummary: null
    stats: null

# Total statistics (accumulated across all phases)
totalStats:
  turns: 45
  inputTokens: 100500
  outputTokens: 69300
  costUsd: 2.35

# Execution timing
execution:
  startTime: "2026-02-10T10:00:00Z"
  endTime: null

# Pull request information (populated after PR phase)
pullRequest:
  url: null  # e.g., "https://github.com/user/repo/pull/123"
  number: null
  title: null
  createdAt: null
  merged: false

# Resume information (for interrupted executions)
resume:
  canResume: true
  lastCompletedPhase: "test"
  nextPhase: "verification"
  interruptedAt: "2026-02-10T12:30:00Z"
  interruptReason: "userCancelled"  # userCancelled, timeout, error, systemShutdown

# Error information (if failed)
error: null
```

## 5. Key Workflows

### 5.1 Init Workflow

```
User: $ gba init
  │
  ├─▶ Check if .gba exists
  │   └─▶ If exists and not --force: Error
  │
  ├─▶ Create .gba directory structure
  │   ├─▶ Create .gba/ directory
  │   ├─▶ Create .trees/ directory
  │   └─▶ Create default config.yml
  │
  ├─▶ Analyze repository (using init task)
  │   ├─▶ Load prompts/init/config.yml
  │   ├─▶ Load prompts/init/system.md + user.md
  │   ├─▶ Execute with Claude Agent SDK
  │   └─▶ AI generates:
  │       ├─▶ .gba.md (repository documentation)
  │       └─▶ Updates CLAUDE.md (if exists)
  │
  ├─▶ Update .gitignore
  │   └─▶ Add .trees/ to .gitignore
  │
  └─▶ Success message
```

**Output**:
```
$ gba init
Initializing GBA for current project...
✓ Created .gba/ directory
✓ Created .trees/ directory
✓ Analyzed repository structure
✓ Generated .gba.md
✓ Updated CLAUDE.md
✓ Updated .gitignore
Done! Project initialized.
```

### 5.2 Plan Workflow

```
User: $ gba plan <feature-slug>
  │
  ├─▶ Check if .gba exists
  │   └─▶ If not: Error "Run gba init first"
  │
  ├─▶ Generate feature ID
  │   └─▶ Get next sequential ID (0001, 0002, etc.)
  │
  ├─▶ Create feature directory
  │   └─▶ .gba/<id>_<feature-slug>/
  │       ├─▶ specs/
  │       └─▶ docs/
  │
  ├─▶ Create git worktree (if enabled in config)
  │   ├─▶ Create branch: feature/<id>-<slug>
  │   ├─▶ Create worktree in .trees/<id>_<slug>/
  │   └─▶ Switch to worktree directory
  │
  ├─▶ Load plan task configuration
  │   ├─▶ Load prompts/plan/config.yml
  │   ├─▶ Load prompts/plan/system.md + user.md
  │   └─▶ Render with context (repo_path, feature_slug, feature_id)
  │
  ├─▶ Start interactive planning session (ratatui TUI)
  │   │
  │   ├─▶ Execute agent with plan prompts
  │   │   └─▶ Agent asks clarifying questions
  │   │
  │   ├─▶ Display chat interface
  │   │   ├─▶ User answers questions
  │   │   └─▶ Agent refines plan
  │   │
  │   ├─▶ Continue conversation until plan complete
  │   │
  │   └─▶ Agent generates:
  │       ├─▶ specs/design.md (architecture & implementation)
  │       └─▶ specs/verification.md (test criteria)
  │
  ├─▶ Save plan artifacts
  │   ├─▶ Write specs/design.md
  │   ├─▶ Write specs/verification.md
  │   └─▶ Create state.yml (status: planned)
  │
  └─▶ Display success message
      └─▶ "Plan finished. Run 'gba run <id>_<slug>' to execute"
```

**Interactive Planning Session**:
```
$ gba plan add-user-auth
Creating feature 0001_add-user-auth...
✓ Created feature directory
✓ Created git worktree (branch: feature/0001-add-user-auth)

Starting interactive planning session...

┌─────────────────────────────────────────────────────────────┐
│ Feature Planning: add-user-auth                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Assistant: Can you describe the authentication feature?    │
│                                                             │
│ You: I want to add JWT-based authentication with login     │
│      and registration endpoints                            │
│                                                             │
│ Assistant: I'll design a solution with:                    │
│            - JWT token generation and validation           │
│            - Login/register endpoints                      │
│            - Middleware for protected routes               │
│            Should I proceed with this approach?            │
│                                                             │
│ You: Yes, looks good                                       │
│                                                             │
│ Assistant: ✓ Generated specs/design.md                    │
│            ✓ Generated specs/verification.md              │
│            Plan complete!                                  │
│                                                             │
└─────────────────────────────────────────────────────────────┘

Plan finished. Run 'gba run 0001_add-user-auth' to execute.
```

### 5.3 Run Workflow

**Supports Resume**: If execution is interrupted (Ctrl+C, network issues, system crash), running `gba run` again automatically resumes from the last completed phase.

```
User: $ gba run <id>_<feature-slug> [--resume]
  │
  ├─▶ Check if .gba exists
  │   └─▶ If not: Error "Run gba init first"
  │
  ├─▶ Load feature state from state.yml
  │   └─▶ If not exists: Error "Feature not found"
  │
  ├─▶ Switch to git worktree (if enabled)
  │   └─▶ cd .trees/<id>_<slug>/
  │
  ├─▶ Check for resume
  │   ├─▶ If --resume flag OR state.resume.canResume:
  │   │   ├─▶ Load currentPhase index
  │   │   ├─▶ Display resume information
  │   │   └─▶ Start from currentPhase
  │   └─▶ Else: Start from phase 0
  │
  ├─▶ For each phase (from start index):
  │   │
  │   ├─▶ Load task configuration
  │   │   ├─▶ Load prompts/{phaseName}/config.yml
  │   │   ├─▶ Load prompts/{phaseName}/system.md + user.md
  │   │   └─▶ Render with context (repo_path, feature_slug, feature_id)
  │   │
  │   ├─▶ Update state.yml
  │   │   ├─▶ phase.status = "in_progress"
  │   │   ├─▶ currentPhase = index
  │   │   └─▶ phase.startedAt = now
  │   │
  │   ├─▶ Display phase header (progress UI)
  │   │
  │   ├─▶ Execute agent (gba-core)
  │   │   ├─▶ Apply task config (preset, tools)
  │   │   ├─▶ Track turns, tokens, and cost
  │   │   ├─▶ Show progress spinner
  │   │   └─▶ Stream output to UI
  │   │
  │   ├─▶ Save phase results
  │   │   ├─▶ Update state.yml:
  │   │   │   ├─▶ phase.status = "completed"
  │   │   │   ├─▶ phase.stats (turns, tokens, cost)
  │   │   │   ├─▶ phase.completedAt = now
  │   │   │   └─▶ phase.outputSummary = "..."
  │   │   └─▶ docs/impl_details.md (append)
  │   │
  │   ├─▶ Git commit (if autoCommit enabled)
  │   │   ├─▶ git add .
  │   │   ├─▶ git commit -m "Phase <name>: <summary>"
  │   │   └─▶ Save commitSha to state.yml
  │   │
  │   ├─▶ Handle errors/interruptions
  │   │   ├─▶ If failed:
  │   │   │   ├─▶ Update state.yml (status = "failed")
  │   │   │   ├─▶ Set resume.canResume = true
  │   │   │   └─▶ Prompt to retry or abort
  │   │   └─▶ If interrupted (Ctrl+C):
  │   │       ├─▶ Update state.yml:
  │   │       │   ├─▶ resume.canResume = true
  │   │       │   ├─▶ resume.lastCompletedPhase = "..."
  │   │       │   ├─▶ resume.nextPhase = "..."
  │   │       │   ├─▶ resume.interruptedAt = now
  │   │       │   └─▶ resume.interruptReason = "userCancelled"
  │   │       └─▶ Display resume instructions
  │   │
  │   └─▶ Continue to next phase
  │
  ├─▶ After PR phase:
  │   └─▶ Update state.yml:
  │       ├─▶ status = "completed"
  │       ├─▶ execution.endTime = now
  │       ├─▶ pullRequest.url = "..."
  │       ├─▶ pullRequest.number = N
  │       ├─▶ pullRequest.createdAt = now
  │       └─▶ currentPhase = phases.len()
  │
  └─▶ Display completion summary
      ├─▶ Show all phases completed
      ├─▶ Show total turns, tokens, and cost
      ├─▶ Show artifacts created
      └─▶ Show PR link
```

**Resume Behavior**:
- When interrupted, state.yml is updated with resume information
- Next run with `--resume` or automatic detection continues from current_phase
- Prompts include context about previous execution and what was completed
- All statistics (turns, tokens, cost) are accumulated across resume sessions

**Git Worktree Benefits**:
- Complete isolation between features
- Can work on multiple features simultaneously
- Clean commit history per feature
- No conflicts with main working directory
```

## 6. Error Handling Strategy

### 6.1 Error Types

```rust
// gba-core errors
#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    #[error("Agent execution failed: {0}")]
    AgentExecutionFailed(String),

    #[error("Agent timeout after {0:?}")]
    AgentTimeout(Duration),

    #[error("Invalid execution context: {0}")]
    InvalidContext(String),

    #[error("Claude SDK error: {0}")]
    SdkError(#[from] claude_agent_sdk_rs::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// gba-pm errors
#[derive(thiserror::Error, Debug)]
pub enum PromptError {
    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("Template render error: {0}")]
    RenderError(String),

    #[error("Template syntax error: {0}")]
    SyntaxError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// gba-cli errors
#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("GBA not initialized. Run 'gba init' first")]
    NotInitialized,

    #[error("Feature not found: {0}")]
    FeatureNotFound(String),

    #[error("Invalid feature slug: {0}")]
    InvalidFeatureSlug(String),

    #[error("Core error: {0}")]
    Core(#[from] gba_core::CoreError),

    #[error("Prompt error: {0}")]
    Prompt(#[from] gba_pm::PromptError),

    #[error("User cancelled operation")]
    UserCancelled,
}
```

### 6.2 Error Recovery

- **Agent timeout**: Prompt user to retry or increase timeout
- **Agent execution failure**: Save partial results, allow resume
- **Template errors**: Validate templates on init, fail fast
- **Network errors**: Retry with exponential backoff
- **User cancellation**: Clean up partial state, save progress

## 7. Development Plan

### Phase 1: Foundation (Week 1)

**Tasks**:
1. Set up workspace structure with Rust 2024 edition
2. Configure dependencies in workspace Cargo.toml
3. Implement basic error types for all crates
4. Set up CI/CD with clippy, fmt, and tests

**Deliverables**:
- Compilable workspace
- Error type definitions
- CI configuration

### Phase 2: gba-pm Implementation (Week 1-2)

**Tasks**:
1. Implement `PromptManager` with minijinja integration
2. Implement `PromptContext` with builder pattern
3. Add template caching
4. Write unit tests for template rendering
5. Create default prompt templates

**Deliverables**:
- Working prompt manager
- Default templates
- Unit tests with >80% coverage

### Phase 3: gba-core Implementation (Week 2-3)

**Tasks**:
1. Implement `AgentRunner` with Claude SDK integration
2. Implement `ExecutionContext` and `ExecutionRequest`
3. Add actor-based execution model with tokio
4. Implement phase execution logic
5. Add timeout and cancellation handling
6. Write unit and integration tests

**Deliverables**:
- Working agent runner
- Phase execution engine
- Integration tests

### Phase 4: gba-cli Init Command (Week 3)

**Tasks**:
1. Implement CLI structure with clap
2. Implement `init` command
3. Create directory structure creation logic
4. Add .gitignore update logic
5. Write integration tests

**Deliverables**:
- Working `gba init` command
- Integration tests

### Phase 5: gba-cli Plan Command (Week 4)

**Tasks**:
1. Implement `plan` command
2. Build interactive UI with ratatui
3. Integrate gba-core and gba-pm
4. Implement conversation flow
5. Add plan artifact saving
6. Write integration tests

**Deliverables**:
- Working `gba plan` command
- Interactive planning UI
- Integration tests

### Phase 6: gba-cli Run Command (Week 5)

**Tasks**:
1. Implement `run` command
2. Build phase execution UI with progress bars
3. Implement resume functionality
4. Add dry-run mode
5. Implement result saving and reporting
6. Write integration tests

**Deliverables**:
- Working `gba run` command
- Phase execution UI
- Integration tests

### Phase 7: Polish and Documentation (Week 6)

**Tasks**:
1. Add comprehensive error messages
2. Write user documentation
3. Create example workflows
4. Performance optimization
5. Security audit
6. Final testing and bug fixes

**Deliverables**:
- Complete documentation
- Example projects
- Production-ready release

## 8. Testing Strategy

### 8.1 Unit Tests

- Test each public function in isolation
- Mock external dependencies (Claude SDK, filesystem)
- Use `rstest` for parameterized tests
- Aim for >80% code coverage

### 8.2 Integration Tests

- Test command workflows end-to-end
- Use temporary directories for filesystem operations
- Mock Claude API responses
- Test error scenarios

### 8.3 Manual Testing

- Test with real Claude API
- Test on different repositories
- Test with various feature types
- Performance testing with large repos

## 9. Security Considerations

- Store API keys in environment variables only
- Never log or expose API keys
- Validate all user input (feature slugs, paths)
- Sanitize template context to prevent injection
- Use `secrecy` crate for API key handling
- Audit dependencies regularly with `cargo audit`

## 10. Performance Considerations

- Cache compiled templates in gba-pm
- Use async I/O for all file operations
- Stream agent output instead of buffering
- Use efficient data structures (DashMap for concurrent access)
- Minimize allocations in hot paths
- Profile with `cargo flamegraph` before optimization

## 11. Future Enhancements

- Support for multiple AI providers (OpenAI, Anthropic, local models)
- Plugin system for custom phases
- Web UI for planning and monitoring
- Team collaboration features
- Integration with GitHub/GitLab APIs
- Metrics and analytics dashboard
- Template marketplace
