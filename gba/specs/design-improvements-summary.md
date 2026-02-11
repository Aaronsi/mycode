# Design.md Improvements Summary

## Date: 2026-02-11

## Overview

Based on comparison with `design_by_tyrchen.md`, the current `design.md` has been significantly improved with the following enhancements.

## Key Improvements Made

### 1. ✅ Feature ID System

**Added**: Sequential feature IDs (0001, 0002, etc.)

**Before**:
```
.gba/features/<feature-slug>/
```

**After**:
```
.gba/features/0001_<feature-slug>/
.gba/features/0002_<feature-slug>/
```

**Benefits**:
- Clear ordering of features
- Unique identification
- Easy reference
- Sortable by creation order

### 2. ✅ Git Worktree Support

**Added**: `.trees/` directory for git worktrees

**Structure**:
```
.trees/
├── 0001_<feature-slug>/  # Isolated working directory
└── 0002_<feature-slug>/
```

**Benefits**:
- Complete isolation between features
- No conflicts with main working directory
- Can work on multiple features simultaneously
- Clean commit history per feature

### 3. ✅ Enhanced State Tracking

**Added to state.yml**:
- Feature ID and slug
- Current phase index (for precise resume)
- Git information (worktree_path, branch, base_branch, base_commit)
- Token tracking (input_tokens, output_tokens)
- Commit SHA per phase
- Total statistics with tokens
- PR merged status
- Error information

**Before**:
```yaml
phases:
  - name: "observe"
    turns: 8
    cost_usd: 0.42
```

**After**:
```yaml
feature:
  id: "0001"
  slug: "user-authentication"

current_phase: 2

git:
  worktree_path: ".trees/0001_user-authentication"
  branch: "feature/0001-user-authentication"
  commit_sha: "abc1234"

phases:
  - name: "observe"
    commit_sha: "def5678"
    stats:
      turns: 8
      input_tokens: 12500
      output_tokens: 8300
      cost_usd: 0.42

total_stats:
  turns: 45
  input_tokens: 100500
  output_tokens: 69300
  cost_usd: 2.35
```

### 4. ✅ Enhanced Configuration

**Added to config.yaml**:
- Agent configuration section
  - permission_mode (auto | manual | none)
  - budget_limit
- Prompts configuration
  - include paths for custom prompts
- Git configuration
  - auto_commit option
  - branch_pattern with variables
  - use_worktree option
  - base_branch
- Review configuration
  - enabled flag
  - provider (codex | claude | none)

**Before**:
```yaml
claude_api_key_env: "ANTHROPIC_API_KEY"
default_model: "claude-sonnet-4-5"
timeout_seconds: 300
```

**After**:
```yaml
agent:
  api_key_env: "ANTHROPIC_API_KEY"
  model: "claude-sonnet-4-5"
  permission_mode: "auto"
  budget_limit: null
  timeout_seconds: 300

prompts:
  include:
    - ~/.config/gba/prompts

git:
  auto_commit: true
  branch_pattern: "feature/{id}-{slug}"
  use_worktree: true
  base_branch: "main"

review:
  enabled: true
  provider: "codex"
```

### 5. ✅ Repository Documentation

**Added**: `.gba.md` generation during init

**Content**:
- Repository overview
- Directory structure
- Key technologies
- Development guidelines

**Benefits**:
- Automatic documentation
- Consistent format
- Easy for AI to understand
- Updated CLAUDE.md reference

### 6. ✅ Enhanced Data Structures

**Added Structures**:
```rust
pub struct ExecutionStats {
    pub turns: u32,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cost_usd: f64,
}

pub struct FeatureInfo {
    pub id: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
```

**Updated Structures**:
- `ExecutionResult` now includes `ExecutionStats`
- `FeatureState` now includes `FeatureInfo`, `GitInfo`, `current_phase`, `total_stats`, `ExecutionTiming`
- `PhaseState` now includes `commit_sha` and `ExecutionStats`
- `PullRequestInfo` now includes `merged` flag

### 7. ✅ Enhanced Workflows

**Init Workflow**:
- Added repository analysis
- Added .gba.md generation
- Added CLAUDE.md update
- Added .trees/ directory creation

**Plan Workflow**:
- Added feature ID generation
- Added git worktree creation
- Added branch creation

**Run Workflow**:
- Added git worktree switching
- Added current_phase index tracking
- Added auto-commit per phase
- Added commit SHA tracking
- Added token tracking
- Enhanced resume with phase index

## Comparison Summary

| Feature | Before | After | Status |
|---------|--------|-------|--------|
| Feature ID | No | Yes (0001, 0002) | ✅ Added |
| Git Worktree | No | Yes (.trees/) | ✅ Added |
| Token Tracking | No | Yes (input/output) | ✅ Added |
| Commit SHA | No | Yes (per phase) | ✅ Added |
| Phase Index | No | Yes (current_phase) | ✅ Added |
| .gba.md | No | Yes (auto-generated) | ✅ Added |
| Permission Mode | No | Yes (auto/manual/none) | ✅ Added |
| Budget Limit | No | Yes (optional) | ✅ Added |
| Auto Commit | No | Yes (configurable) | ✅ Added |
| Branch Pattern | No | Yes (with variables) | ✅ Added |
| Review Provider | No | Yes (codex/claude) | ✅ Added |
| Custom Prompts | No | Yes (include paths) | ✅ Added |
| PR Merged | No | Yes (boolean) | ✅ Added |
| Error Info | No | Yes (in state.yml) | ✅ Added |

## Benefits of Improvements

### For Users
1. ✅ Better feature organization with IDs
2. ✅ Work on multiple features simultaneously (worktrees)
3. ✅ Detailed cost tracking (tokens)
4. ✅ Budget control
5. ✅ Automatic documentation
6. ✅ Flexible configuration

### For Developers
1. ✅ Clear feature identification
2. ✅ Complete git integration
3. ✅ Detailed statistics for analysis
4. ✅ Better resume capability
5. ✅ Extensible configuration

### For System
1. ✅ Production-ready design
2. ✅ Scalable architecture
3. ✅ Professional features
4. ✅ Industry best practices

## Implementation Priority

### P0 - Critical (Must Have)
- [x] Feature ID system
- [x] Enhanced state tracking (tokens, commits, phase index)
- [x] Enhanced configuration structure
- [x] .gba.md generation

### P1 - Important (Should Have)
- [x] Git worktree support
- [x] Auto-commit per phase
- [x] Branch pattern configuration
- [x] Review provider configuration

### P2 - Nice to Have
- [ ] Streaming API (future enhancement)
- [ ] Session management (future enhancement)

## Backward Compatibility

**Breaking Changes**:
- Feature directory structure changed from `<slug>` to `<id>_<slug>`
- Config structure changed (nested sections)
- State structure enhanced (more fields)

**Migration Path**:
- Existing features can be migrated by adding ID prefix
- Config can be migrated with script
- State can be migrated by adding new fields with defaults

## Documentation Updates

- [x] Updated design.md with all improvements
- [x] Created design-comparison.md
- [x] Created this summary document

## Next Steps

1. **Implement in Code**
   - Implement FeatureState with new structure
   - Implement GitInfo and worktree management
   - Implement enhanced configuration loading
   - Implement .gba.md generation

2. **Update Templates**
   - Update init.md to include .gba.md generation
   - Update prompts to reference feature IDs

3. **Test Thoroughly**
   - Test feature ID generation
   - Test git worktree creation
   - Test token tracking
   - Test auto-commit
   - Test resume with phase index

---

**Status**: ✅ Design improvements complete
**Version**: 2.1 (with tyrchen's enhancements)
**Quality**: Production-ready
**Ready For**: Implementation
