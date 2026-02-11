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

1. Verify git status and ensure working directory is clean
2. Create feature branch if needed: `feature/{{ feature_slug }}`
3. Stage and commit changes with conventional commits format
4. Push to remote
5. Create PR using `gh pr create` with comprehensive description

### Commit Message Format

```
feat({{ feature_slug }}): brief description

Detailed description of what was implemented and why.

- Key change 1
- Key change 2

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>
```

### PR Description Template

Include:
- Overview of the feature
- Changes made (files created/modified/deleted)
- Implementation details (architecture, components, decisions)
- Testing (coverage, results, manual testing)
- Verification (requirements met, quality checks)
- Breaking changes (if any)
- Documentation updates
- Checklist (code quality, tests, docs)

## Git Commands

```bash
git status
git checkout -b feature/{{ feature_slug }}
git add .
git commit -m "feat({{ feature_slug }}): description"
git push -u origin feature/{{ feature_slug }}
gh pr create --title "feat: {{ feature_slug }}" --body "..." --label "enhancement"
```

## Output Requirements

After creating the PR, provide:

1. **PR Information**: URL, number, branch, title, status
2. **Commit Information**: SHA, message, files changed
3. **Summary**: What was implemented, key changes, test results
4. **Next Steps**: Wait for review, address comments, merge after approval

Begin PR creation now.
