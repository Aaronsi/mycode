# GBA Initialization

## Repository: {{ repo_path }}

You are initializing the Geektime Bootcamp Agent (GBA) for this repository.

## Your Task

Set up the GBA directory structure and configuration files to enable AI-assisted feature development.

### Initialization Steps

1. **Create Directory Structure**
   ```
   .gba/
   ├── config.yaml
   ├── prompts/
   │   ├── plan.md
   │   ├── phase_1_observe.md
   │   ├── phase_2_build.md
   │   ├── phase_3_test.md
   │   ├── phase_4_verification.md
   │   ├── phase_5_review.md
   │   └── phase_6_pr.md
   ├── features/
   └── .trees/              # Temporary execution trees
   ```

2. **Create Configuration File**
   Create `.gba/config.yaml` with default settings

3. **Copy Prompt Templates**
   Copy all prompt templates to `.gba/prompts/`

4. **Update .gitignore**
   Add the following lines to `.gitignore` (create if doesn't exist):
   ```
   # GBA execution trees (temporary files)
   .gba/features/*/trees/
   .gba/.trees/
   ```

5. **Verify Setup**
   - Check all directories created
   - Check all files created
   - Verify .gitignore updated

## Configuration Template

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

## Important Notes

- **DO** create all directories and files
- **DO** update .gitignore
- **DO** verify setup is complete
- **DO NOT** modify existing project files (except .gitignore)
- **DO NOT** create any feature directories yet (those are created during `gba plan`)

## Output Requirements

After initialization, provide:

1. **Setup Summary**
   ```markdown
   # GBA Initialization Complete

   ## Created Directories
   - .gba/
   - .gba/prompts/
   - .gba/features/

   ## Created Files
   - .gba/config.yaml

   ## Updated Files
   - .gitignore (added GBA exclusions)

   ## Status
   ✅ GBA initialized successfully

   ## Next Steps
   Run `gba plan <feature-slug>` to start planning a new feature.
   ```

2. **Verification**
   - Confirm all directories exist
   - Confirm config.yaml is valid
   - Confirm .gitignore is updated

Begin initialization now.
