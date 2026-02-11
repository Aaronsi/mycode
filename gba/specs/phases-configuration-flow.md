# Phases Configuration and Execution Flow

## Overview

The phases in GBA are configured in `.gba/config.yaml` and loaded by the execution engine during `gba run`.

## Phase Configuration Source

### Location
`.gba/config.yaml`

### Structure
```yaml
version: "0.1.0"
claude_api_key_env: "ANTHROPIC_API_KEY"
default_model: "claude-sonnet-4-5"
timeout_seconds: 300

phases:
  - name: "observe"
    prompt_template: "phase_1_observe.md"
    description: "Observe codebase and understand context"

  - name: "build"
    prompt_template: "phase_2_build.md"
    description: "Build implementation"

  - name: "test"
    prompt_template: "phase_3_test.md"
    description: "Write and run tests"

  - name: "verification"
    prompt_template: "phase_4_verification.md"
    description: "Verify implementation against requirements"

  - name: "review"
    prompt_template: "phase_5_review.md"
    description: "Code review and refinement"

  - name: "pr"
    prompt_template: "phase_6_pr.md"
    description: "Create pull request"
```

## Phase Loading Flow

### 1. Initialization (`gba init`)

```
User: $ gba init
  │
  ├─▶ Create .gba/config.yaml
  │   └─▶ Write default phase configuration
  │
  └─▶ Copy prompt templates to .gba/prompts/
      ├─▶ phase_1_observe.md
      ├─▶ phase_2_build.md
      ├─▶ phase_3_test.md
      ├─▶ phase_4_verification.md
      ├─▶ phase_5_review.md
      └─▶ phase_6_pr.md
```

### 2. Execution (`gba run`)

```
User: $ gba run <feature-slug>
  │
  ├─▶ Load .gba/config.yaml
  │   └─▶ Parse phases configuration
  │
  ├─▶ Load state.yml (if exists)
  │   └─▶ Check for resume information
  │
  ├─▶ Determine starting phase
  │   ├─▶ If resuming: start from resume.next_phase
  │   └─▶ Else: start from first phase
  │
  └─▶ Execute phases sequentially
      └─▶ For each phase:
          ├─▶ Load prompt template from .gba/prompts/{prompt_template}
          ├─▶ Render with PromptContext
          ├─▶ Execute agent
          ├─▶ Update state.yml
          └─▶ Continue to next phase
```

## Data Structures

### Configuration (Rust)

```rust
#[derive(Debug, Clone, serde::Deserialize)]
pub struct GbaConfig {
    pub version: String,
    pub claude_api_key_env: String,
    pub default_model: String,
    pub timeout_seconds: u64,
    pub phases: Vec<PhaseConfig>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PhaseConfig {
    pub name: String,
    pub prompt_template: String,
    pub description: String,
}

impl GbaConfig {
    /// Load configuration from .gba/config.yaml
    pub fn load(repo_path: &Path) -> Result<Self> {
        let config_path = repo_path.join(".gba/config.yaml");
        let content = std::fs::read_to_string(config_path)?;
        let config: GbaConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Get default configuration (used during init)
    pub fn default() -> Self {
        Self {
            version: "0.1.0".to_string(),
            claude_api_key_env: "ANTHROPIC_API_KEY".to_string(),
            default_model: "claude-sonnet-4-5".to_string(),
            timeout_seconds: 300,
            phases: vec![
                PhaseConfig {
                    name: "observe".to_string(),
                    prompt_template: "phase_1_observe.md".to_string(),
                    description: "Observe codebase and understand context".to_string(),
                },
                PhaseConfig {
                    name: "build".to_string(),
                    prompt_template: "phase_2_build.md".to_string(),
                    description: "Build implementation".to_string(),
                },
                PhaseConfig {
                    name: "test".to_string(),
                    prompt_template: "phase_3_test.md".to_string(),
                    description: "Write and run tests".to_string(),
                },
                PhaseConfig {
                    name: "verification".to_string(),
                    prompt_template: "phase_4_verification.md".to_string(),
                    description: "Verify implementation against requirements".to_string(),
                },
                PhaseConfig {
                    name: "review".to_string(),
                    prompt_template: "phase_5_review.md".to_string(),
                    description: "Code review and refinement".to_string(),
                },
                PhaseConfig {
                    name: "pr".to_string(),
                    prompt_template: "phase_6_pr.md".to_string(),
                    description: "Create pull request".to_string(),
                },
            ],
        }
    }
}
```

### Phase Execution (Rust)

```rust
// In gba-cli/src/commands/run.rs

pub async fn run_command(feature_slug: String, resume: bool) -> Result<()> {
    let repo_path = std::env::current_dir()?;

    // 1. Load configuration
    let config = GbaConfig::load(&repo_path)?;

    // 2. Load feature state
    let feature_path = repo_path.join(format!(".gba/features/{}", feature_slug));
    let mut state = FeatureState::load(&feature_path)
        .unwrap_or_else(|_| FeatureState::new(feature_slug.clone()));

    // 3. Determine starting phase
    let start_index = if resume && state.resume.can_resume {
        config.phases.iter()
            .position(|p| Some(&p.name) == state.resume.next_phase.as_ref())
            .unwrap_or(0)
    } else {
        0
    };

    // 4. Execute phases
    for phase_config in &config.phases[start_index..] {
        println!("Executing phase: {}", phase_config.name);

        // Load prompt template
        let template_path = repo_path.join(format!(".gba/prompts/{}", phase_config.prompt_template));

        // Build context
        let mut context = PromptContext::load(&feature_path, &repo_path)?;
        context = context.with_phase(phase_config.name.clone());

        // Add resume info if resuming
        if resume && start_index > 0 {
            let resume_context = ResumeContext {
                last_completed_phase: state.resume.last_completed_phase.clone().unwrap_or_default(),
                interrupted_at: state.resume.interrupted_at.map(|t| t.to_string()).unwrap_or_default(),
                interrupt_reason: format!("{:?}", state.resume.interrupt_reason),
                completed_phases: state.phases.iter()
                    .filter(|p| p.status == PhaseStatus::Completed)
                    .map(|p| p.name.clone())
                    .collect(),
            };
            context = context.with_resume_info(resume_context);
        }

        // Render prompt
        let prompt_manager = PromptManager::new(repo_path.join(".gba/prompts"))?;
        let prompt = prompt_manager.render(&phase_config.prompt_template, &context)?;

        // Execute agent
        let agent_runner = AgentRunner::new(api_key)?;
        let result = agent_runner.execute(ExecutionRequest {
            prompt,
            context: ExecutionContext {
                repo_path: repo_path.clone(),
                feature_slug: feature_slug.clone(),
                phase_name: Some(phase_config.name.clone()),
                metadata: HashMap::new(),
            },
            timeout: Some(Duration::from_secs(config.timeout_seconds)),
        }).await?;

        // Update state
        state.update_phase(&phase_config.name, PhaseStatus::Completed, &result);
        state.save(&feature_path)?;

        // Update previous_output for next phase
        context = context.with_previous_output(result.output.clone());
    }

    println!("All phases completed!");
    Ok(())
}
```

## Phase Customization

### Users Can Customize Phases

Users can modify `.gba/config.yaml` to:

1. **Add custom phases**
```yaml
phases:
  - name: "observe"
    prompt_template: "phase_1_observe.md"
    description: "Observe codebase"

  - name: "custom_analysis"
    prompt_template: "custom_analysis.md"
    description: "Custom analysis step"

  - name: "build"
    prompt_template: "phase_2_build.md"
    description: "Build implementation"
```

2. **Reorder phases**
```yaml
phases:
  - name: "build"
    prompt_template: "phase_2_build.md"
    description: "Build first"

  - name: "observe"
    prompt_template: "phase_1_observe.md"
    description: "Then observe"
```

3. **Skip phases**
```yaml
phases:
  - name: "observe"
    prompt_template: "phase_1_observe.md"
    description: "Observe"

  - name: "build"
    prompt_template: "phase_2_build.md"
    description: "Build"

  # Skip test phase

  - name: "pr"
    prompt_template: "phase_6_pr.md"
    description: "Create PR"
```

4. **Use custom templates**
```yaml
phases:
  - name: "observe"
    prompt_template: "my_custom_observe.md"  # Custom template
    description: "Custom observation"
```

## Template Resolution

### Template Loading Priority

1. **Project-specific templates**: `.gba/prompts/{template_name}`
2. **Default templates**: `crates/gba-pm/templates/{template_name}` (embedded in binary)

```rust
impl PromptManager {
    pub fn new(template_dir: PathBuf) -> Result<Self> {
        let mut env = Environment::new();

        // Set up loader with fallback
        env.set_loader(|name| {
            // Try project-specific template first
            let project_path = template_dir.join(name);
            if project_path.exists() {
                return std::fs::read_to_string(project_path).ok();
            }

            // Fall back to embedded default template
            DEFAULT_TEMPLATES.get(name).map(|s| s.to_string())
        });

        Ok(Self { env, template_dir })
    }
}
```

## Summary

### Phase Configuration Flow

```
gba init
  └─▶ Creates .gba/config.yaml with default phases
  └─▶ Copies default templates to .gba/prompts/

gba run <feature>
  ├─▶ Loads .gba/config.yaml
  │   └─▶ Reads phases array
  │
  ├─▶ For each phase:
  │   ├─▶ Loads template from .gba/prompts/{prompt_template}
  │   ├─▶ Renders with PromptContext
  │   ├─▶ Executes agent
  │   └─▶ Updates state.yml
  │
  └─▶ Completes when all phases done
```

### Key Points

1. ✅ **Phases are configured in `.gba/config.yaml`**
2. ✅ **Default configuration created during `gba init`**
3. ✅ **Loaded by execution engine during `gba run`**
4. ✅ **Users can customize phases**
5. ✅ **Templates resolved from `.gba/prompts/` or defaults**
6. ✅ **State tracked in `state.yml` per phase**

---

**Status**: Phase configuration and loading flow documented
**Location**: `.gba/config.yaml`
**Customizable**: Yes
