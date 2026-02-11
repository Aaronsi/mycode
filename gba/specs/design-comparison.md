# Design Comparison: Current vs Tyrchen's Design

## Executive Summary

This document compares the current `design.md` with `design_by_tyrchen.md` to identify strengths, weaknesses, and areas for improvement.

## Key Differences Overview

| Aspect | Current Design | Tyrchen's Design |
|--------|---------------|------------------|
| **Feature ID** | No ID system | Sequential ID (0001, 0002) |
| **Git Worktree** | Not used | Uses .trees/ with worktrees |
| **State Tracking** | Basic (turns, cost) | Detailed (tokens, commits) |
| **Config Format** | YAML | YAML (more detailed) |
| **Phase Definition** | 6 phases | Flexible phases |
| **Resume** | Basic | Advanced with phase index |
| **Documentation** | .gba.md not mentioned | Generates .gba.md |

## Detailed Comparison

### 1. Directory Structure

#### Current Design
```
.gba/
├── config.yaml
├── prompts/
└── features/<slug>/
    ├── specs/
    ├── docs/
    ├── state.yml
    └── trees/
```

**Pros:**
- ✅ Simple and clean
- ✅ Easy to understand
- ✅ Feature-centric organization

**Cons:**
- ❌ No feature ID system
- ❌ No git worktree isolation
- ❌ No .gba.md documentation

#### Tyrchen's Design
```
.gba/
├── config.yml
└── 0001_<slug>/
    ├── specs/
    ├── docs/
    └── state.yml
.trees/
└── 0001_<slug>/  # Git worktree
.gba.md  # Repository documentation
```

**Pros:**
- ✅ Sequential feature IDs (0001, 0002)
- ✅ Git worktree isolation
- ✅ Automatic .gba.md generation
- ✅ Clear feature ordering

**Cons:**
- ⚠️ More complex setup
- ⚠️ Requires git worktree knowledge

### 2. State Tracking

#### Current Design (state.yml)
```yaml
execution:
  total_turns: 45
  total_cost_usd: 2.35

phases:
  - name: "observe"
    turns: 8
    cost_usd: 0.42
```

**Pros:**
- ✅ Simple and focused
- ✅ Tracks essential metrics (turns, cost)
- ✅ Easy to parse

**Cons:**
- ❌ No token tracking (input/output)
- ❌ No commit SHA tracking
- ❌ No phase index for resume
- ❌ Less detailed statistics

#### Tyrchen's Design (state.yml)
```yaml
current_phase: 2  # Phase index

git:
  worktree_path: .trees/0001_add-user-auth
  branch: feature/0001-add-user-auth
  commit_sha: abc1234

phases:
  - name: setup
    stats:
      turns: 5
      input_tokens: 12500
      output_tokens: 8300
      cost_usd: 0.15
    commit_sha: abc1234

total_stats:
  turns: 20
  input_tokens: 65500
  output_tokens: 45800
  cost_usd: 0.83
```

**Pros:**
- ✅ Detailed token tracking
- ✅ Git integration (commit SHA, branch)
- ✅ Phase index for precise resume
- ✅ Comprehensive statistics

**Cons:**
- ⚠️ More complex structure
- ⚠️ Requires git worktree management

### 3. Configuration

#### Current Design (config.yaml)
```yaml
version: "0.1.0"
claude_api_key_env: "ANTHROPIC_API_KEY"
default_model: "claude-sonnet-4-5"
timeout_seconds: 300

phases:
  - name: "observe"
    prompt_template: "phase_1_observe.md"
    description: "..."
```

**Pros:**
- ✅ Simple and minimal
- ✅ Easy to understand
- ✅ Focuses on essentials

**Cons:**
- ❌ No permission mode configuration
- ❌ No budget limit
- ❌ No git auto-commit option
- ❌ No review provider configuration
- ❌ No prompt include paths

#### Tyrchen's Design (config.yml)
```yaml
agent:
  permission_mode: auto
  budget_limit: 10.0

prompts:
  include:
    - ~/.config/gba/prompts

git:
  auto_commit: true
  branch_pattern: "feature/{id}-{slug}"

review:
  enabled: true
  provider: codex
```

**Pros:**
- ✅ Comprehensive configuration
- ✅ Permission mode control
- ✅ Budget limits
- ✅ Git automation
- ✅ Review provider options
- ✅ Custom prompt paths

**Cons:**
- ⚠️ More complex
- ⚠️ More options to learn

### 4. Feature Identification

#### Current Design
- Uses `<feature-slug>` only
- No sequential numbering
- Example: `user-authentication`

**Pros:**
- ✅ Simple
- ✅ Human-readable

**Cons:**
- ❌ No ordering
- ❌ Hard to reference
- ❌ No unique ID

#### Tyrchen's Design
- Uses `<id>_<slug>` format
- Sequential numbering (0001, 0002)
- Example: `0001_user-authentication`

**Pros:**
- ✅ Clear ordering
- ✅ Unique ID
- ✅ Easy to reference
- ✅ Sortable

**Cons:**
- ⚠️ Slightly more complex

### 5. Git Integration

#### Current Design
- No git worktree
- Works in main working directory
- Manual branch management

**Pros:**
- ✅ Simple
- ✅ No worktree complexity
- ✅ Familiar workflow

**Cons:**
- ❌ No isolation between features
- ❌ Risk of conflicts
- ❌ No automatic branch creation

#### Tyrchen's Design
- Uses git worktree in `.trees/`
- Automatic branch creation
- Isolated working directories
- Auto-commit per phase

**Pros:**
- ✅ Complete isolation
- ✅ No conflicts between features
- ✅ Automatic branch management
- ✅ Clean commit history
- ✅ Can work on multiple features simultaneously

**Cons:**
- ⚠️ Requires git worktree knowledge
- ⚠️ More disk space
- ⚠️ More complex setup

### 6. Documentation Generation

#### Current Design
- No automatic documentation
- Manual CLAUDE.md management

**Cons:**
- ❌ No .gba.md generation
- ❌ No repository analysis
- ❌ Manual documentation updates

#### Tyrchen's Design
- Generates `.gba.md` during init
- Analyzes repository structure
- Updates CLAUDE.md with reference

**Pros:**
- ✅ Automatic documentation
- ✅ Repository analysis
- ✅ Consistent format
- ✅ Easy for AI to understand

### 7. API Design

#### Current Design
```rust
pub struct AgentRunner {
    pub async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResult>;
    pub async fn execute_phases(&self, phases: Vec<Phase>) -> Result<Vec<ExecutionResult>>;
}
```

**Pros:**
- ✅ Simple API
- ✅ Clear purpose

**Cons:**
- ❌ No streaming support
- ❌ No session management
- ❌ No event handlers

#### Tyrchen's Design
```rust
pub struct Engine {
    pub async fn run(&self, task: Task) -> Result<TaskResult>;
    pub async fn run_stream(&self, task: Task, handler: impl EventHandler) -> Result<TaskResult>;
    pub fn session(&self) -> Session;
}

pub struct Session {
    pub async fn send(&mut self, message: &str) -> Result<String>;
    pub async fn send_stream(&mut self, message: &str, handler: impl EventHandler) -> Result<String>;
}
```

**Pros:**
- ✅ Streaming support
- ✅ Session management
- ✅ Event handlers
- ✅ More flexible

## Recommendations

### Critical Improvements Needed

1. **✅ Add Feature ID System**
   - Use `<id>_<slug>` format
   - Sequential numbering (0001, 0002)
   - Better organization and reference

2. **✅ Enhanced State Tracking**
   - Add token tracking (input/output)
   - Add commit SHA per phase
   - Add phase index for precise resume
   - Add git information

3. **✅ Git Worktree Support**
   - Use `.trees/` for isolation
   - Automatic branch creation
   - Auto-commit per phase
   - Better multi-feature workflow

4. **✅ Generate .gba.md**
   - Automatic repository documentation
   - Repository structure analysis
   - Update CLAUDE.md reference

5. **✅ Enhanced Configuration**
   - Add permission_mode
   - Add budget_limit
   - Add git auto_commit option
   - Add review provider config
   - Add custom prompt paths

### Optional Improvements

6. **⚠️ Streaming API**
   - Add run_stream() method
   - Add EventHandler trait
   - Better progress feedback

7. **⚠️ Session Management**
   - Add Session struct
   - Support multi-turn conversations
   - Better for plan command

## Priority Ranking

### Must Have (P0)
1. ✅ Feature ID system (`0001_<slug>`)
2. ✅ Enhanced state tracking (tokens, commits, phase index)
3. ✅ Generate .gba.md during init

### Should Have (P1)
4. ✅ Git worktree support
5. ✅ Enhanced configuration options
6. ✅ Auto-commit per phase

### Nice to Have (P2)
7. ⚠️ Streaming API with EventHandler
8. ⚠️ Session management for interactive planning

## Conclusion

**Tyrchen's design is more comprehensive and production-ready**, with better:
- Feature organization (IDs)
- Git integration (worktrees)
- State tracking (tokens, commits)
- Configuration options
- Documentation generation

**Current design is simpler** but lacks:
- Feature IDs
- Git isolation
- Detailed tracking
- Automatic documentation

### Recommended Action

**Update current design.md to incorporate:**
1. Feature ID system
2. Enhanced state tracking
3. Git worktree support
4. .gba.md generation
5. Enhanced configuration

This will make GBA more robust, professional, and production-ready while maintaining simplicity where possible.

---

**Next Step**: Update design.md with these improvements
