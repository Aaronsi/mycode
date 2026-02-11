# Design Specification Updates - Phase Management and State Tracking

## Date: 2026-02-10

## Summary of Changes

This document summarizes the updates made to the GBA design specification to add comprehensive phase management, state tracking, and resume capabilities.

## 1. Added Verification Phase

### Change
Added a new phase `verification` between `test` and `review` phases.

### Updated Phase Sequence
1. **observe** - Observe codebase and understand context
2. **build** - Build implementation
3. **test** - Write and run tests
4. **verification** - Verify implementation against requirements ✨ NEW
5. **review** - Code review and refinement
6. **pr** - Create pull request

### Rationale
Verification phase ensures that all requirements from the design specification are met before code review. This provides a systematic check against acceptance criteria.

## 2. Added state.yml for Execution Tracking

### Structure
```yaml
version: "0.1.0"
feature_slug: "feature-name"
status: "in_progress"  # pending, in_progress, completed, failed
created_at: "2026-02-10T10:00:00Z"
updated_at: "2026-02-10T12:30:00Z"

execution:
  total_turns: 45
  total_cost_usd: 2.35
  start_time: "2026-02-10T10:00:00Z"
  end_time: null

phases:
  - name: "observe"
    status: "completed"
    turns: 8
    cost_usd: 0.42
    started_at: "2026-02-10T10:00:00Z"
    completed_at: "2026-02-10T10:15:00Z"
    output_summary: "Summary of phase output"

pull_request:
  url: "https://github.com/user/repo/pull/123"
  number: 123
  title: "feat: feature-name"
  created_at: "2026-02-10T13:00:00Z"

resume:
  can_resume: true
  last_completed_phase: "test"
  next_phase: "verification"
  interrupted_at: "2026-02-10T12:30:00Z"
  interrupt_reason: "user_cancelled"
```

### Features
- **Execution Statistics**: Tracks turns and cost per phase and total
- **Phase History**: Complete history of all phase executions
- **PR Information**: Stores PR URL and metadata after creation
- **Resume Information**: Enables resuming interrupted executions

### Location
`.gba/features/<feature-slug>/state.yml`

## 3. Added Resume/Interrupt Handling

### Interrupt Reasons
- `user_cancelled` - User pressed Ctrl+C
- `timeout` - Execution timeout
- `error` - Unrecoverable error
- `system_shutdown` - System shutdown

### Resume Workflow
1. User runs `gba run <feature-slug> --resume` or automatic detection
2. System loads `state.yml`
3. Checks `resume.can_resume` flag
4. Loads `resume.last_completed_phase`
5. Starts from `resume.next_phase`
6. Passes resume context to prompts

### Resume Context in Prompts
All phase prompts now include:
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

## 4. Updated Data Structures

### ExecutionResult
Added fields:
- `turns: u32` - Number of turns in this execution
- `cost_usd: f64` - Cost in USD for this execution

### PromptContext
Added fields:
- `verification_criteria: Option<String>` - Verification criteria from specs
- `resume_info: Option<ResumeContext>` - Resume information if resuming

### New Types
- `FeatureState` - Complete feature state
- `ExecutionStats` - Execution statistics
- `PhaseState` - Individual phase state
- `PullRequestInfo` - PR metadata
- `ResumeInfo` - Resume information
- `ResumeContext` - Resume context for prompts

## 5. Created Complete Prompt Templates

All templates created in `crates/gba-pm/templates/`:

1. **init.md** - GBA initialization
2. **plan.md** - Interactive feature planning
3. **phase_1_observe.md** - Codebase observation and analysis
4. **phase_2_build.md** - Implementation
5. **phase_3_test.md** - Testing
6. **phase_4_verification.md** - Verification against requirements ✨ NEW
7. **phase_5_review.md** - Code review and refinement
8. **phase_6_pr.md** - Pull request creation

### Template Features
- All templates in English
- Include resume information handling
- Comprehensive instructions
- Clear output requirements
- Verification checklists
- Examples and best practices

## 6. Updated Run Workflow

### Enhanced Workflow
- Load feature state from `state.yml`
- Check for resume capability
- Display resume information if resuming
- Track turns and cost for each phase
- Update `state.yml` after each phase
- Handle interruptions gracefully
- Save PR information after PR phase

### State Updates
After each phase:
- Update phase status
- Record turns and cost
- Save output summary
- Update timestamps
- Set resume information if interrupted

## 7. File Structure Updates

### Updated Structure
```
.gba/features/<feature-slug>/
├── specs/
│   ├── design.md
│   └── verification.md
├── docs/
│   └── impl_details.md
├── state.yml              # ✨ NEW - Execution state
└── trees/
    ├── plan.json
    ├── phase_1.json
    └── ...
```

## Implementation Checklist

### Phase 1: gba-core Updates
- [ ] Implement `FeatureState` struct and methods
- [ ] Add `load()` and `save()` for state.yml
- [ ] Update `ExecutionResult` with turns and cost
- [ ] Implement state tracking in phase execution
- [ ] Add interrupt handling (Ctrl+C)
- [ ] Implement resume logic

### Phase 2: gba-pm Updates
- [ ] Update `PromptContext` with new fields
- [ ] Add `ResumeContext` struct
- [ ] Update template rendering to include resume info
- [ ] Add verification_criteria to context

### Phase 3: gba-cli Updates
- [ ] Add `--resume` flag to run command
- [ ] Implement state loading and checking
- [ ] Display resume information to user
- [ ] Handle Ctrl+C gracefully
- [ ] Update state.yml during execution
- [ ] Display execution statistics

### Phase 4: Templates
- [x] Create all 8 prompt templates
- [x] Add resume information sections
- [x] Add verification checklists
- [x] Add comprehensive instructions

## Benefits

1. **Cost Tracking**: Users can see exactly how much each phase costs
2. **Resume Capability**: Interrupted executions can be resumed without starting over
3. **Progress Tracking**: Clear visibility into execution progress
4. **Verification Phase**: Systematic verification against requirements
5. **PR Tracking**: PR information stored for reference
6. **Audit Trail**: Complete history of execution

## Testing Requirements

1. Test normal execution flow (all phases)
2. Test interruption and resume
3. Test state.yml persistence
4. Test cost and turn tracking
5. Test verification phase
6. Test PR information storage
7. Test error handling and recovery

## Documentation Updates

- [x] Updated design.md with new structures
- [x] Added state.yml specification
- [x] Updated run workflow
- [x] Created all prompt templates
- [x] Added resume handling documentation

---

**Status**: ✅ Design specification updated
**Next Step**: Implement changes in code
