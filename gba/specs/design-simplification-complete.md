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

**After**: `prompts/` (root level)

**Rationale**:
- Templates are project-level resources, not crate-specific
- Each task has its own `config.yml` in `.gba/feature-name/`
- Shared templates should be at project root
- Clearer separation: code in `crates/`, resources at root

**New Structure**:
```
gba/
├── prompts/              # Project-level prompt templates
│   ├── init/
│   │   ├── system.md
│   │   └── user.md
│   ├── plan/
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
└── .gba/                # Per-task workspace
    └── feature-name/
        ├── config.yml   # Task-specific config
        └── .trees/
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

```yaml
# config.yml
version: "0.1.0"

# Model configuration
model:
  name: "claude-sonnet-4-5"
  max_turns: 10
  provider: "codex"

# Phase configuration
# Templates automatically loaded from prompts/{phase_name}/system.md and user.md
phases:
  - name: "observe"
    description: "Observe codebase and understand context"
    preset: false        # Use custom system.md
    tools: []           # All tools available

  - name: "build"
    description: "Build implementation"
    preset: false        # Use custom system.md (Rust developer role)
    tools: []           # All tools available

  - name: "test"
    description: "Write and run tests"
    preset: false        # Use custom system.md (test engineer role)
    tools: []           # All tools available

  - name: "verification"
    description: "Verify implementation against requirements"
    preset: false        # Use custom system.md (QA role)
    tools: []           # All tools available

  - name: "review"
    description: "Code review and refinement"
    preset: false        # Use custom system.md (reviewer role)
    tools: []           # All tools available

  - name: "pr"
    description: "Create pull request"
    preset: true         # Use claude_code preset
    tools: ["Bash"]     # Only Bash for git operations
```

## Implementation Guide

### PromptManager Updates

```rust
impl PromptManager {
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
```

### AgentRunner Updates

```rust
impl AgentRunner {
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

            // Load prompts based on phase configuration
            let (system_prompt, user_prompt) = if phase.preset {
                // Use claude_code preset
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
                system_prompt,
                user_prompt,
                tools: phase.tools.clone(),
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
