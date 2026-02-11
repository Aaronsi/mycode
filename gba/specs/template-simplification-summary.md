# Template Simplification Summary

## Date: 2026-02-11

## Overview

All prompt templates have been simplified following the principle of "convention over configuration". This reduces complexity and makes the system more maintainable.

## Key Changes

### 1. Removed Optional Variables

**Before**: Templates had many optional variables with conditionals
```jinja2
{% if extra.readme %}
### README
{{ extra.readme }}
{% endif %}
```

**After**: All variables are always provided (empty string if file doesn't exist)
```jinja2
### README
{{ readme }}
```

### 2. Removed Template Functions

**Before**: Templates used functions like `list_files()` and `read_file()`
```jinja2
{% for file in list_files(repo_path, "**/*.rs") %}
- {{ file }}
{% endfor %}

{{ read_file(repo_path ~ "/CLAUDE.md") }}
```

**After**: Agent uses tools directly, variables provided by engine
```markdown
Use the Glob tool to explore the codebase.

{{ coding_standards }}
```

### 3. Simplified Context Loading

**Before**: Manual builder pattern with many optional fields
```rust
PromptContext::new(repo, feature)
    .with_specs(specs)
    .with_extra("readme", readme)
    .with_extra("coding_standards", standards)
```

**After**: Automatic loading from standard locations
```rust
PromptContext::load(feature_path, repo_path)?
    .with_previous_output(output)
    .with_resume_info(resume)
```

## Final Variable List

### Always Provided (Required)

| Variable | Source | Convention |
|----------|--------|------------|
| `repo_path` | Context | Repository root path |
| `feature_slug` | Context | Feature identifier |
| `specs` | File | `<feature>/specs/design.md` |
| `verification_criteria` | File | `<feature>/specs/verification.md` |
| `previous_output` | State | Previous phase output |
| `coding_standards` | File | `<repo>/CLAUDE.md` |
| `readme` | File | `<repo>/README.md` |

### Conditionally Provided (Optional)

| Variable | Source | When Present |
|----------|--------|--------------|
| `resume_info.*` | state.yml | Only when resuming |

## Removed Variables

| Variable | Reason | Alternative |
|----------|--------|-------------|
| `description` | Not needed | Always ask user |
| `extra.*` | Too flexible | Specific variables |
| `phase` | Not used | Implicit from template |

## Removed Functions

| Function | Reason | Alternative |
|----------|--------|-------------|
| `list_files()` | Performance | Use Glob tool |
| `read_file()` | Complexity | Provide as variable |

## Benefits

1. ✅ **Simpler Templates**
   - No complex conditionals
   - Easier to read and maintain
   - Less error-prone

2. ✅ **Convention Over Configuration**
   - Standard file locations
   - Predictable behavior
   - Less configuration needed

3. ✅ **Better Performance**
   - No expensive operations in templates
   - Pre-loaded context
   - Cached when possible

4. ✅ **Clearer Contracts**
   - Engine knows exactly what to provide
   - Templates know exactly what's available
   - No surprises

5. ✅ **Easier Testing**
   - Fewer variables to mock
   - Predictable inputs
   - Simpler test cases

## Implementation Requirements

### PromptContext Changes

```rust
pub struct PromptContext {
    // Required fields (always present)
    pub repo_path: String,
    pub feature_slug: String,
    pub specs: String,
    pub verification_criteria: String,
    pub previous_output: String,
    pub coding_standards: String,
    pub readme: String,

    // Optional fields
    pub phase: Option<String>,
    pub resume_info: Option<ResumeContext>,
}

impl PromptContext {
    /// Load all context from standard locations
    pub fn load(feature_path: &Path, repo_path: &Path) -> Result<Self>;

    /// Add resume info when resuming
    pub fn with_resume_info(mut self, resume: ResumeContext) -> Self;

    /// Set previous output
    pub fn with_previous_output(mut self, output: String) -> Self;

    /// Set phase name
    pub fn with_phase(mut self, phase: String) -> Self;
}
```

### File Loading Convention

The engine automatically reads from these locations:

```
<repo>/
├── README.md              → readme
├── CLAUDE.md              → coding_standards
└── .gba/features/<slug>/
    └── specs/
        ├── design.md      → specs
        └── verification.md → verification_criteria
```

If a file doesn't exist, the variable contains an empty string (not an error).

## Updated Templates

All templates have been updated:

1. ✅ `plan.md` - Removed description conditional, list_files function
2. ✅ `phase_1_observe.md` - Simplified context sections
3. ✅ `phase_2_build.md` - Removed files_to_modify/create, simplified coding_standards
4. ✅ `phase_3_test.md` - No changes needed (already simple)
5. ✅ `phase_4_verification.md` - Removed verification_criteria conditional
6. ✅ `phase_5_review.md` - Simplified coding_standards
7. ✅ `phase_6_pr.md` - No changes needed (already simple)
8. ✅ `init.md` - No changes needed (already simple)

## Migration Guide

### For Template Authors

**Old way**:
```jinja2
{% if extra.some_field %}
{{ extra.some_field }}
{% else %}
Default value
{% endif %}
```

**New way**:
```jinja2
{{ some_field }}
```

### For Engine Implementers

**Old way**:
```rust
let mut context = PromptContext::new(repo, feature);
if let Some(readme) = read_readme() {
    context = context.with_extra("readme", readme);
}
```

**New way**:
```rust
let context = PromptContext::load(feature_path, repo_path)?;
```

## Testing

### Template Testing

Templates are now easier to test:

```rust
let context = PromptContext {
    repo_path: "/repo".to_string(),
    feature_slug: "test".to_string(),
    specs: "# Design".to_string(),
    verification_criteria: "- [ ] Test".to_string(),
    previous_output: "Previous output".to_string(),
    coding_standards: "# Standards".to_string(),
    readme: "# README".to_string(),
    phase: Some("build".to_string()),
    resume_info: None,
};

let rendered = prompt_manager.render("phase_2_build.md", &context)?;
```

### Engine Testing

Engine testing is simpler:

```rust
// Just ensure files are read correctly
let context = PromptContext::load(feature_path, repo_path)?;
assert_eq!(context.specs, expected_specs);
assert_eq!(context.readme, expected_readme);
```

## Backward Compatibility

This is a breaking change for:
- ❌ Custom templates using `extra.*` variables
- ❌ Custom templates using `list_files()` or `read_file()`
- ❌ Code using old PromptContext builder pattern

Migration is straightforward:
1. Remove conditionals from templates
2. Replace `extra.*` with direct variables
3. Replace functions with tool instructions
4. Use `PromptContext::load()` instead of builder

## Documentation Updates

- ✅ Updated `specs/design.md` with new PromptContext
- ✅ Updated all templates
- ✅ Created `specs/template-variables-review.md`
- ✅ Created this summary document

---

**Status**: ✅ Template simplification complete
**Impact**: Breaking change, requires engine updates
**Benefits**: Simpler, faster, more maintainable
