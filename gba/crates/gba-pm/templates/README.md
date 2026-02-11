# GBA Prompt Templates

This directory contains all prompt templates used by GBA for different phases of feature development.

## Template Overview

| Template | Purpose | When Used |
|----------|---------|-----------|
| `init.md` | Initialize GBA in repository | `gba init` |
| `plan.md` | Interactive feature planning | `gba plan <feature>` |
| `phase_1_observe.md` | Observe and analyze codebase | Phase 1 of `gba run` |
| `phase_2_build.md` | Implement the feature | Phase 2 of `gba run` |
| `phase_3_test.md` | Write and run tests | Phase 3 of `gba run` |
| `phase_4_verification.md` | Verify against requirements | Phase 4 of `gba run` |
| `phase_5_review.md` | Code review and refinement | Phase 5 of `gba run` |
| `phase_6_pr.md` | Create pull request | Phase 6 of `gba run` |

## Template Features

All templates include:
- Clear objectives and instructions
- Resume information handling (for interrupted executions)
- Context from previous phases
- Output requirements
- Best practices and guidelines
- Verification checklists

## Template Variables

Templates use Jinja2 syntax with the following variables:

### Common Variables
- `{{ repo_path }}` - Path to the repository
- `{{ feature_slug }}` - Feature identifier (e.g., "user-auth")
- `{{ specs }}` - Design specification content
- `{{ verification_criteria }}` - Verification criteria from specs
- `{{ previous_output }}` - Output from previous phase

### Resume Variables
- `{{ resume_info.last_completed_phase }}` - Last completed phase name
- `{{ resume_info.interrupted_at }}` - Timestamp of interruption
- `{{ resume_info.interrupt_reason }}` - Reason for interruption
- `{{ resume_info.completed_phases }}` - List of completed phases

### Extra Variables
- `{{ extra.readme }}` - Repository README content
- `{{ extra.coding_standards }}` - Project coding standards
- `{{ extra.architecture }}` - Architecture documentation
- `{{ extra.files_to_modify }}` - List of files to modify
- `{{ extra.files_to_create }}` - List of files to create

## Template Workflow

```
gba init
  └─> init.md
      └─> Creates .gba structure

gba plan <feature>
  └─> plan.md
      └─> Interactive planning
      └─> Creates specs/design.md and specs/verification.md

gba run <feature>
  ├─> phase_1_observe.md
  │   └─> Analyzes codebase
  │   └─> Plans file changes
  │
  ├─> phase_2_build.md
  │   └─> Implements feature
  │   └─> Creates/modifies files
  │
  ├─> phase_3_test.md
  │   └─> Writes tests
  │   └─> Runs cargo test
  │   └─> Runs cargo clippy
  │
  ├─> phase_4_verification.md
  │   └─> Verifies requirements
  │   └─> Checks verification criteria
  │   └─> Validates quality
  │
  ├─> phase_5_review.md
  │   └─> Reviews code quality
  │   └─> Refines implementation
  │   └─> Ensures best practices
  │
  └─> phase_6_pr.md
      └─> Creates pull request
      └─> Generates PR description
      └─> Updates state.yml with PR info
```

## Resume Handling

All phase templates (phase_1 through phase_6) include resume handling:

```jinja2
{% if resume_info %}
## Resume Information
**Resuming from interrupted execution**
- Last completed phase: {{ resume_info.last_completed_phase }}
- Interrupted at: {{ resume_info.interrupted_at }}
- Reason: {{ resume_info.interrupt_reason }}
- Completed phases: {{ resume_info.completed_phases | join(", ") }}

Please continue from where you left off.
{% endif %}
```

This allows the agent to understand the context when resuming an interrupted execution.

## Template Guidelines

When modifying templates:

1. **Keep instructions clear and concise**
2. **Include specific examples**
3. **Define clear output requirements**
4. **Add verification checklists**
5. **Handle resume scenarios**
6. **Use consistent formatting**
7. **Write in English**

## Template Testing

To test templates:

1. Initialize GBA: `gba init`
2. Plan a feature: `gba plan test-feature`
3. Run phases: `gba run test-feature`
4. Test interruption: Press Ctrl+C during execution
5. Test resume: `gba run test-feature --resume`

## Custom Templates

To customize templates:

1. Copy template to `.gba/prompts/` in your repository
2. Modify as needed
3. Templates in `.gba/prompts/` override defaults
4. Keep the same filename

## Template Maintenance

- Templates are version controlled in `crates/gba-pm/templates/`
- Updates should maintain backward compatibility
- Test all templates after changes
- Document any breaking changes

## Related Documentation

- [Design Specification](../specs/design.md) - Overall system design
- [Design Updates](../specs/design-updates-phase-management.md) - Recent changes
- [Quick Start](../QUICKSTART.md) - Getting started guide

---

**Version**: 1.0
**Last Updated**: 2026-02-10
**Status**: ✅ Complete
