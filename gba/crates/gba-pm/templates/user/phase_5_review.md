# Phase 5: Code Review and Refinement

## Feature: {{ feature_slug }}
**Repository**: {{ repo_path }}

{% if resume_info %}
## Resume Information
**Resuming from interrupted execution**
- Last completed phase: {{ resume_info.last_completed_phase }}
- Interrupted at: {{ resume_info.interrupted_at }}
- Reason: {{ resume_info.interrupt_reason }}
- Completed phases: {{ resume_info.completed_phases | join(", ") }}

Please continue code review from where you left off.
{% endif %}

## Design Specification

{{ specs }}

## Previous Phase Output

{% if previous_output %}
### Verification Phase Results
{{ previous_output }}
{% else %}
No previous phase output available.
{% endif %}

## Your Task

Perform a thorough code review of the implementation and refine the code to ensure it meets the highest quality standards.

### Review Process

1. **Read Through All Changed Files**
   - Review each file systematically
   - Take notes on issues found
   - Identify areas for improvement

2. **Check Against Standards**
   - Verify compliance with project coding standards
   - Check Rust best practices are followed
   - Ensure consistency with existing code

3. **Identify Issues**
   - Critical issues (must fix)
   - Major issues (should fix)
   - Minor issues (nice to fix)
   - Suggestions for improvement

4. **Make Refinements**
   - Fix critical and major issues
   - Improve code clarity where needed
   - Add missing documentation
   - Optimize performance if needed

5. **Re-verify**
   - Run tests after changes
   - Run clippy after changes
   - Ensure nothing broke

## Project Coding Standards

{{ coding_standards }}

## Output Requirements

After code review and refinement, provide:

1. **Review Summary** with issues found and fixed by category
2. **Improvements Made** with list of refinements
3. **Changes Made** during review
4. **Final Verification** confirming tests pass and no warnings
5. **Sign-off**: "✅ Code review complete. Implementation is ready for PR."

Begin code review now.
