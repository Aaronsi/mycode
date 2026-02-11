# GBA Template Structure Documentation

## Overview

GBA templates are organized by execution steps, with each step containing both system and user prompts.

## Directory Structure

```
crates/gba-pm/templates/
├── init/
│   ├── system.md          # System prompt for initialization
│   └── user.md            # User prompt for initialization task
├── plan/
│   ├── system.md          # System prompt for planning (architect role)
│   └── user.md            # User prompt for planning task
├── observe/
│   ├── system.md          # System prompt for observation
│   └── user.md            # User prompt for observation task
├── build/
│   ├── system.md          # System prompt for building (developer role)
│   └── user.md            # User prompt for building task
├── test/
│   ├── system.md          # System prompt for testing (tester role)
│   └── user.md            # User prompt for testing task
├── verification/
│   ├── system.md          # System prompt for verification (QA role)
│   └── user.md            # User prompt for verification task
├── review/
│   ├── system.md          # System prompt for review (reviewer role)
│   └── user.md            # User prompt for review task
└── pr/
    ├── system.md          # System prompt for PR creation (devops role)
    └── user.md            # User prompt for PR creation task
```

## Configuration Example

In `.gba/config.yaml`:

```yaml
phases:
  - name: "observe"
    system_prompt: "observe/system.md"
    user_prompt: "observe/user.md"

  - name: "build"
    system_prompt: "build/system.md"
    user_prompt: "build/user.md"

  - name: "test"
    system_prompt: "test/system.md"
    user_prompt: "test/user.md"
```

## System Prompts

Each step has a specialized system prompt defining the AI's role:

- **init**: Base system prompt with general capabilities
- **plan**: Software architect role
- **observe**: Code analyst role (uses base)
- **build**: Software developer role
- **test**: Test engineer role
- **verification**: QA engineer role
- **review**: Code reviewer role
- **pr**: DevOps engineer role

## User Prompts

Each step has a user prompt defining the specific task:

- Clear task description
- Context information (repo_path, specs, etc.)
- Task objectives and steps
- Output requirements
- No role definitions (handled by system prompt)

## Benefits

1. **Clear Organization**: Each step is self-contained
2. **Easy Navigation**: Find prompts by step name
3. **Simple Configuration**: Just reference step/system.md and step/user.md
4. **Maintainability**: Update system or user prompts independently

## SDK Integration

```rust
// Load templates for a step
let system_prompt = load_template("observe/system.md")?;
let user_prompt = load_template("observe/user.md")?;

// Render and use
let options = ClaudeAgentOptions {
    system_prompt: Some(SystemPrompt::Text(render(system_prompt, &context)?)),
    ..Default::default()
};
```
