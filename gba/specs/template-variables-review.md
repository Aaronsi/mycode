# Prompt Template Variables Review

## Review Criteria

1. **Necessity**: Is this variable necessary or can we use convention over configuration?
2. **Context Availability**: Can the execution engine provide this context?
3. **Simplicity**: Can we simplify or remove optional variables?

## Variables Analysis

### Core Variables (Always Available)

These are essential and provided by execution engine:

| Variable | Source | Necessity | Keep? |
|----------|--------|-----------|-------|
| `{{ repo_path }}` | Execution context | ✅ Essential | ✅ YES |
| `{{ feature_slug }}` | Execution context | ✅ Essential | ✅ YES |
| `{{ specs }}` | Read from specs/design.md | ✅ Essential | ✅ YES |
| `{{ previous_output }}` | Previous phase result | ✅ Essential | ✅ YES |

### Optional Variables (Need Review)

#### 1. `{{ description }}` in plan.md

**Current Usage**:
```jinja2
{% if description %}
{{ description }}
{% else %}
Please ask the user to describe the feature they want to implement.
{% endif %}
```

**Analysis**:
- ❌ **Not necessary** - Agent should always ask user for description
- ❌ **Adds complexity** - Conditional logic not needed
- ✅ **Recommendation**: Remove. Always start with asking user.

**Simplified**:
```markdown
## Feature Description

Please ask the user to describe the feature they want to implement.
```

#### 2. `{{ extra.readme }}`, `{{ extra.architecture }}`, `{{ extra.coding_standards }}`

**Current Usage**: Multiple templates check these optionals

**Analysis**:
- ⚠️ **Partially necessary** - Useful context but optional
- ✅ **Can be provided** - Execution engine can read these files
- ✅ **Recommendation**: Keep but simplify - engine should always try to provide

**Convention**:
- Always read README.md if exists
- Always read CLAUDE.md (coding standards) if exists
- Always read ARCHITECTURE.md if exists

**Simplified**: Remove conditionals, engine provides or empty string

#### 3. `{{ verification_criteria }}` in phase_4_verification.md

**Current Usage**:
```jinja2
{% if verification_criteria %}
{{ verification_criteria }}
{% else %}
- [ ] Requirement 1: [description]
{% endif %}
```

**Analysis**:
- ✅ **Necessary** - Core to verification phase
- ✅ **Can be provided** - Read from specs/verification.md
- ❌ **Conditional not needed** - Should always exist after plan phase
- ✅ **Recommendation**: Remove conditional, always provide

#### 4. `{{ extra.files_to_modify }}`, `{{ extra.files_to_create }}`

**Current Usage**: In phase_2_build.md

**Analysis**:
- ⚠️ **Questionable** - Observation phase should document this
- ✅ **Can be provided** - From phase 1 output
- ❌ **Conditional not needed** - Should be in previous_output
- ✅ **Recommendation**: Remove. Use previous_output instead.

#### 5. `{{ resume_info }}` and all resume fields

**Current Usage**: All phase templates

**Analysis**:
- ✅ **Necessary** - Core feature for resume
- ✅ **Can be provided** - From state.yml
- ✅ **Conditional needed** - Only present when resuming
- ✅ **Recommendation**: Keep as is

### Functions Used

#### 1. `list_files(repo_path, "**/*.rs")`

**Current Usage**: In plan.md

**Analysis**:
- ⚠️ **Questionable** - Can be very long, not always useful
- ❌ **Performance issue** - Slow on large repos
- ✅ **Recommendation**: Remove. Agent can use Glob tool instead.

#### 2. `read_file(repo_path ~ "/CLAUDE.md")`

**Current Usage**: In phase_2_build.md

**Analysis**:
- ✅ **Necessary** - Provides coding standards
- ✅ **Can be provided** - Engine can read and provide as variable
- ✅ **Recommendation**: Change to variable `{{ coding_standards }}`

## Recommended Simplifications

### 1. Remove Optional Description in plan.md

**Before**:
```jinja2
{% if description %}
{{ description }}
{% else %}
Please ask the user to describe the feature they want to implement.
{% endif %}
```

**After**:
```markdown
Please ask the user to describe the feature they want to implement.
```

### 2. Remove list_files() Function

**Before**:
```jinja2
{% for file in list_files(repo_path, "**/*.rs") %}
- {{ file }}
{% endfor %}
```

**After**:
```markdown
Use the Glob tool to explore the codebase structure.
```

### 3. Simplify Context Sections

**Before**:
```jinja2
{% if extra.readme %}
### README
{{ extra.readme }}
{% endif %}
```

**After**:
```markdown
### README
{{ readme }}
```
(Engine always provides, empty if not exists)

### 4. Remove files_to_modify/create

**Before**:
```jinja2
{% if extra.files_to_modify %}
{% for file in extra.files_to_modify %}
- {{ file }}
{% endfor %}
{% endif %}
```

**After**:
```markdown
Refer to the observation phase output for files to modify.
```

### 5. Simplify verification_criteria

**Before**:
```jinja2
{% if verification_criteria %}
{{ verification_criteria }}
{% else %}
- [ ] Requirement 1: [description]
{% endif %}
```

**After**:
```markdown
{{ verification_criteria }}
```
(Always provided from specs/verification.md)

## Final Variable List

### Required Variables (Always Provided)

| Variable | Source | Description |
|----------|--------|-------------|
| `repo_path` | Context | Repository path |
| `feature_slug` | Context | Feature identifier |
| `specs` | specs/design.md | Design specification |
| `verification_criteria` | specs/verification.md | Verification criteria |
| `previous_output` | Previous phase | Output from previous phase |
| `coding_standards` | CLAUDE.md | Project coding standards |
| `readme` | README.md | Project README |

### Optional Variables (Conditional)

| Variable | Source | When Present |
|----------|--------|--------------|
| `resume_info.*` | state.yml | When resuming execution |

### Removed Variables

| Variable | Reason |
|----------|--------|
| `description` | Not needed, always ask user |
| `extra.readme` | Simplified to `readme` |
| `extra.coding_standards` | Simplified to `coding_standards` |
| `extra.architecture` | Removed, agent can read if needed |
| `extra.files_to_modify` | Use previous_output instead |
| `extra.files_to_create` | Use previous_output instead |

### Removed Functions

| Function | Reason |
|----------|--------|
| `list_files()` | Performance issue, use Glob tool |
| `read_file()` | Replaced with variables |

## Implementation in Execution Engine

The execution engine should:

1. **Always provide core variables**:
   - Read specs/design.md → `specs`
   - Read specs/verification.md → `verification_criteria`
   - Read CLAUDE.md → `coding_standards`
   - Read README.md → `readme`
   - Get previous phase output → `previous_output`

2. **Conditionally provide resume info**:
   - Check state.yml for resume capability
   - If resuming, populate `resume_info`

3. **Handle missing files gracefully**:
   - If file doesn't exist, provide empty string
   - No need for conditionals in templates

## Benefits of Simplification

1. ✅ **Simpler templates** - Less conditional logic
2. ✅ **Convention over configuration** - Standard file locations
3. ✅ **Easier to maintain** - Fewer variables to track
4. ✅ **Better performance** - No expensive operations in templates
5. ✅ **Clearer contracts** - Engine knows exactly what to provide

## Next Steps

1. Update all templates to remove unnecessary variables
2. Update PromptContext struct to match final variable list
3. Update execution engine to provide all required variables
4. Test with simplified templates

---

**Status**: Analysis complete, ready for template updates
