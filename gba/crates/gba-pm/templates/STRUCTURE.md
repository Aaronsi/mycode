# GBA Template Structure Documentation

## Overview

GBA templates are now organized into two categories:
- **System Prompts**: Define the AI agent's role, capabilities, and behavior
- **User Prompts**: Define the specific task and context for each phase

## Directory Structure

```
crates/gba-pm/templates/
├── system/                    # System prompts (role definitions)
│   ├── base.md               # Base system prompt (all phases)
│   ├── architect.md          # Software architect role (plan phase)
│   ├── developer.md          # Developer role (build phase)
│   ├── tester.md            # Test engineer role (test phase)
│   ├── qa.md                # QA engineer role (verification phase)
│   ├── reviewer.md          # Code reviewer role (review phase)
│   └── devops.md            # DevOps engineer role (PR phase)
└── user/                     # User prompts (task definitions)
    ├── init.md              # Initialization task
    ├── plan.md              # Planning task
    ├── phase_1_observe.md   # Observation task
    ├── phase_2_build.md     # Implementation task
    ├── phase_3_test.md      # Testing task
    ├── phase_4_verification.md  # Verification task
    ├── phase_5_review.md    # Review task
    └── phase_6_pr.md        # PR creation task
```

## Configuration Example

In `.gba/config.yaml`:

```yaml
phases:
  - name: "observe"
    system_prompt: "system/base.md"
    user_prompt: "user/phase_1_observe.md"

  - name: "build"
    system_prompt: "system/developer.md"
    user_prompt: "user/phase_2_build.md"

  - name: "test"
    system_prompt: "system/tester.md"
    user_prompt: "user/phase_3_test.md"
```

## SDK Integration

```rust
let system_prompt = SystemPrompt::Text(rendered_system_prompt);
let options = ClaudeAgentOptions {
    system_prompt: Some(system_prompt),
    ..Default::default()
};
```

## Benefits

1. **Clarity**: Role definitions separate from task descriptions
2. **Reusability**: System prompts shared across similar tasks
3. **Maintainability**: Update roles or tasks independently
4. **Best Practice**: Follows prompt engineering standards
