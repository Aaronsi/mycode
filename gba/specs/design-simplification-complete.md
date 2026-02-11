# GBA Design Simplification Complete

## Completion Time
2026-02-11

## Changes Made

### 1. Simplified Preset Logic ✅

**Before**: Complex matrix of preset usage per phase with tool restrictions

**After**: Simple boolean flag
```yaml
phases:
  - name: "build"
    preset: false    # Use custom system.md
    tools: []       # Empty = all tools

  - name: "pr"
    preset: true     # Use claude_code preset
    tools: ["Bash"] # Only Bash tool
```

**Implementation**:
```rust
// Option 1: Custom system prompt
let (system_prompt, user_prompt) = prompt_manager.load_phase_prompts("build", &context)?;
let options = ClaudeAgentOptions {
    system_prompt: Some(SystemPrompt::Text(system_prompt)),
    ..Default::default()
};

// Option 2: Claude Code preset
let options = ClaudeAgentOptions {
    system_prompt: Some(SystemPrompt::Preset(
        SystemPromptPreset::new("claude_code")
    )),
    ..Default::default()
};
```

### 2. Template Location Change ✅

**Before**: `crates/gba-pm/templates/`

**After**: `prompts/` (root level) with per-task configuration

**Rationale**:
- Templates are project-level resources, not crate-specific
- Each task is self-contained with its own `config.yml`
- Shared templates should be at project root
- Clearer separation: code in `crates/`, resources at root
- Follows "convention over configuration" principle

**New Structure**:
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
│   ├── build/
│   ├── test/
│   ├── verification/
│   ├── review/
│   └── pr/
├── crates/
│   ├── gba-core/
│   ├── gba-pm/          # Prompt manager code only
│   └── gba-cli/
└── .gba/                # Per-feature workspace
    └── feature-name/
        └── state.yml    # Feature execution state
```

**Task Configuration** (`prompts/{taskName}/config.yml`):
```yaml
preset: false           # true: use claude_code preset, false: use custom system.md
tools: []              # Empty = all tools, or specify: ["Bash", "Read", "Write"]
disallowedTools: []    # Tools to explicitly disallow (empty = no restrictions)
```

### 3. Vec<T> Instead of Option<Vec<T>> ✅

**Before**: `tools: Option<Vec<String>>`

**After**: `tools: Vec<String>`

**Semantics**:
- Empty vec `[]` = all tools available (default)
- Non-empty vec `["Bash", "Read"]` = only specified tools

**Benefits**:
- Simpler API
- No need to handle `None` case
- Clear default behavior

### 4. Updated Data Structures ✅

#### ExecutionRequest
```rust
pub struct ExecutionRequest {
    pub system_prompt: Option<String>, // None = use claude_code preset
    pub user_prompt: String,
    pub tools: Vec<String>,            // Empty = all tools
    pub context: ExecutionContext,
    pub timeout: Option<Duration>,
}
```

#### Phase
```rust
pub struct Phase {
    pub name: String,
    pub description: String,
    pub preset: bool,              // true = use claude_code preset
    pub tools: Vec<String>,        // Empty = all tools
    pub context: ExecutionContext,
}
```

### 5. Updated Configuration Format ✅

**Important**: All configuration files use camelCase for field names (following `#[serde(rename_all = "camelCase")]`).

#### Project-Level Configuration (`.gba/config.yml`)

Project-level config only contains global settings. Task-specific settings are in each task's `config.yml`.

```yaml
# .gba/config.yml
version: "0.1.0"

# Agent configuration
agent:
  apiKeyEnv: "ANTHROPIC_API_KEY"
  model: "claude-sonnet-4-5"
  permissionMode: "auto"
  budgetLimit: null
  timeoutSeconds: 300

# Git configuration
git:
  autoCommit: true
  branchPattern: "feature/{id}-{slug}"
  useWorktree: true
  baseBranch: "main"

# Phase execution order (config in prompts/{phaseName}/config.yml)
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

#### Task-Level Configuration (`prompts/{taskName}/config.yml`)

Each task directory contains its own configuration:

```yaml
# prompts/build/config.yml
preset: false           # Use custom system.md (Rust developer role)
tools: []              # All tools available
disallowedTools: []    # No restrictions

# prompts/pr/config.yml
preset: true            # Use claude_code preset
tools: ["Bash"]        # Only Bash for git operations
disallowedTools: []

# prompts/observe/config.yml
preset: false           # Use custom analyst role
tools: ["Read", "Glob", "Grep"]  # Only read operations
disallowedTools: ["Write", "Edit", "Bash"]
```

## Implementation Guide

### PromptManager Updates

```rust
impl PromptManager {
    /// Load task configuration from prompts/{taskName}/config.yml
    pub fn load_task_config(&self, task_name: &str) -> Result<TaskConfig> {
        let config_path = self.template_dir.join(task_name).join("config.yml");
        let content = std::fs::read_to_string(&config_path)?;
        let config: TaskConfig = serde_yaml::from_str(&content)?;
        Ok(config)
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
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskConfig {
    pub preset: bool,
    pub tools: Vec<String>,
    pub disallowed_tools: Vec<String>,
}
```

### AgentRunner Updates

```rust
impl AgentRunner {
    pub async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResult> {
        // Load task configuration
        let task_config = self.prompt_manager.load_task_config(&request.task_name)?;

        // Build options based on task configuration
        let mut options = self.inner.options.clone();

        // Set system prompt based on preset flag
        options.system_prompt = if task_config.preset {
            // Use claude_code preset
            Some(SystemPrompt::Preset(
                SystemPromptPreset::new("claude_code")
            ))
        } else {
            // Use custom system prompt
            let (system_prompt, _) = self.prompt_manager.load_phase_prompts(
                &request.task_name,
                &request.context
            )?;
            Some(SystemPrompt::Text(system_prompt))
        };

        // Set tools based on configuration
        if !task_config.tools.is_empty() {
            options.tools = task_config.tools;
        }
        // Note: disallowedTools would need SDK support

        // Create client and execute
        let mut client = ClaudeClient::new(options)?;
        client.connect().await?;
        client.query(&request.user_prompt).await?;

        // ... handle response
    }

    pub async fn execute_phases(&self, phases: Vec<Phase>) -> Result<Vec<ExecutionResult>> {
        let mut results = Vec::with_capacity(phases.len());
        let executor = AgentExecutorHandle::new(self.clone());

        for (idx, phase) in phases.into_iter().enumerate() {
            tracing::info!("Executing phase {}: {}", idx + 1, phase.name);

            // Load task configuration
            let task_config = self.prompt_manager.load_task_config(&phase.name)?;

            // Load prompts
            let (system_prompt, user_prompt) = if task_config.preset {
                // Use preset, only load user prompt
                let user_prompt = self.prompt_manager.render(
                    &format!("{}/user.md", phase.name),
                    &phase.context
                )?;
                (None, user_prompt) // None = use preset
            } else {
                // Use custom system prompt
                let (sys, usr) = self.prompt_manager.load_phase_prompts(
                    &phase.name,
                    &phase.context
                )?;
                (Some(sys), usr)
            };

            let request = ExecutionRequest {
                task_name: phase.name.clone(),
                system_prompt,
                user_prompt,
                tools: task_config.tools,
                context: phase.context.clone(),
                timeout: Some(Duration::from_secs(300)),
            };

            let result = executor.execute(request).await?;

            if !result.success {
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

## Benefits

### 1. Simplicity
- Boolean preset flag instead of complex logic
- Clear semantics: `preset: true/false`
- Easy to understand and configure

### 2. Flexibility
- Empty tools vec = all tools (sensible default)
- Specific tools when needed (e.g., PR phase only needs Bash)
- Custom system prompts for specialized roles
- Claude Code preset for standard operations

### 3. Maintainability
- Templates at project root, easy to find
- Code in crates, resources at root
- Clear separation of concerns
- Convention over configuration

### 4. Type Safety
- `Vec<T>` instead of `Option<Vec<T>>`
- Simpler API surface
- Fewer edge cases to handle

## Next Steps

1. ✅ Update design.md - **Completed**
2. ⏳ Move templates to `prompts/` directory
3. ⏳ Implement `PromptManager::load_phase_prompts()`
4. ⏳ Update `AgentRunner::execute()` with preset logic
5. ⏳ Update `AgentRunner::execute_phases()` with new API
6. ⏳ Update `PhaseConfig` structure
7. ⏳ Update default config.yml template in init phase
8. ⏳ Test new implementation
9. ⏳ Update user documentation

## Summary

✅ **Design Simplification Complete**

Successfully simplified the GBA design with:
- Simple boolean preset flag (no complex matrix)
- Templates moved to root `prompts/` directory
- `Vec<T>` instead of `Option<Vec<T>>` for cleaner API
- Updated data structures and configuration format
- Clear implementation guide for code changes

The design is now simpler, more maintainable, and easier to understand while maintaining full flexibility for different use cases.

---

**Completion Date**: 2026-02-11
**Status**: ✅ Design Complete, Implementation Pending
