# Phase 6: Create Pull Request

## Feature: {{ feature_slug }}
**Repository**: {{ repo_path }}

{% if resume_info %}
## Resume Information
**Resuming from interrupted execution**
- Last completed phase: {{ resume_info.last_completed_phase }}
- Interrupted at: {{ resume_info.interrupted_at }}
- Reason: {{ resume_info.interrupt_reason }}
- Completed phases: {{ resume_info.completed_phases | join(", ") }}

Please continue PR creation from where you left off.
{% endif %}

## Design Specification

{{ specs }}

## Previous Phase Output

{% if previous_output %}
### Review Phase Results
{{ previous_output }}
{% else %}
No previous phase output available.
{% endif %}

## Your Task

Create a pull request for the implemented feature with comprehensive documentation and context.

### PR Creation Steps

1. **Verify Git Status**
   - Check current branch
   - Verify all changes are committed
   - Ensure working directory is clean

2. **Create Feature Branch** (if not already on one)
   - Branch name format: `feature/{{ feature_slug }}`
   - Create from main/master branch

3. **Stage and Commit Changes**
   - Stage all relevant files
   - Create meaningful commit message
   - Follow conventional commits format

4. **Push to Remote**
   - Push feature branch to remote
   - Verify push succeeded

5. **Create Pull Request**
   - Use `gh pr create` command
   - Provide comprehensive PR description
   - Add appropriate labels
   - Request reviewers if needed

### Commit Message Format

Follow conventional commits format:

```
feat({{ feature_slug }}): brief description

Detailed description of what was implemented and why.

- Key change 1
- Key change 2
- Key change 3

Closes #issue_number (if applicable)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>
```

### PR Description Template

```markdown
# Feature: {{ feature_slug }}

## Overview
[Brief description of the feature]

## Changes Made

### New Files
- `path/to/file.rs`: [purpose]

### Modified Files
- `path/to/file.rs`: [changes made]

### Deleted Files
- `path/to/file.rs`: [reason for deletion]

## Implementation Details

### Architecture
[Brief architecture overview]

### Key Components
- Component 1: [description]
- Component 2: [description]

### Design Decisions
- Decision 1: [rationale]
- Decision 2: [rationale]

## Testing

### Test Coverage
- Unit tests: [number] tests
- Integration tests: [number] tests
- Test coverage: [percentage]

### Test Results
```
cargo test output:
[summary of test results]
```

### Manual Testing
- [x] Tested scenario 1
- [x] Tested scenario 2

## Verification

All verification criteria met:
- [x] Functional requirement 1
- [x] Functional requirement 2
- [x] Performance requirements met
- [x] Security considerations addressed
- [x] Documentation complete

## Code Quality

- [x] All tests pass
- [x] No clippy warnings
- [x] Code formatted with rustfmt
- [x] All public APIs documented
- [x] No unwrap/expect in production code

## Breaking Changes

[List any breaking changes, or state "None"]

## Migration Guide

[If breaking changes, provide migration guide, or state "Not applicable"]

## Documentation

- [x] README updated (if needed)
- [x] API documentation complete
- [x] Examples provided
- [x] Architecture docs updated (if needed)

## Checklist

- [x] Code follows project style guidelines
- [x] Self-review completed
- [x] Comments added for complex logic
- [x] Documentation updated
- [x] Tests added/updated
- [x] All tests pass
- [x] No new warnings

## Related Issues

Closes #[issue_number]

## Screenshots/Examples

[If applicable, add screenshots or example usage]

## Additional Notes

[Any additional context or notes for reviewers]
```

## Git Commands

Use these commands to create the PR:

```bash
# Check current status
git status

# Create and checkout feature branch (if needed)
git checkout -b feature/{{ feature_slug }}

# Stage all changes
git add .

# Commit with message
git commit -m "feat({{ feature_slug }}): [brief description]

[Detailed description]

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"

# Push to remote
git push -u origin feature/{{ feature_slug }}

# Create PR using GitHub CLI
gh pr create \
  --title "feat: {{ feature_slug }}" \
  --body "[PR description from template above]" \
  --label "enhancement" \
  --label "ai-generated"
```

## Important Notes

- **DO** write a comprehensive PR description
- **DO** include all relevant context
- **DO** mention any breaking changes
- **DO** provide test results
- **DO** follow conventional commits format
- **DO NOT** create PR if tests are failing
- **DO NOT** skip documentation
- **DO NOT** forget to push changes

## Output Requirements

After creating the PR, provide:

1. **PR Information**
   ```markdown
   # Pull Request Created

   - **PR URL**: [URL]
   - **PR Number**: #[number]
   - **Branch**: feature/{{ feature_slug }}
   - **Title**: feat: {{ feature_slug }}
   - **Status**: Open
   - **Created At**: [timestamp]
   ```

2. **Commit Information**
   ```
   Commit SHA: [sha]
   Commit Message: [message]
   Files Changed: [number]
   Insertions: [number]
   Deletions: [number]
   ```

3. **Summary**
   - What was implemented
   - Key changes made
   - Test results
   - Any notes for reviewers

4. **Next Steps**
   - Wait for code review
   - Address review comments if any
   - Merge after approval

Begin PR creation now. Use Bash tool to run git commands and create the pull request.
