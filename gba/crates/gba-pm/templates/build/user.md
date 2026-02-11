# Phase 2: Build Implementation

## Feature: {{ feature_slug }}
**Repository**: {{ repo_path }}

{% if resume_info %}
## Resume Information
**Resuming from interrupted execution**
- Last completed phase: {{ resume_info.last_completed_phase }}
- Interrupted at: {{ resume_info.interrupted_at }}
- Reason: {{ resume_info.interrupt_reason }}
- Completed phases: {{ resume_info.completed_phases | join(", ") }}

Please continue the implementation from where you left off. Review what was already completed and proceed with remaining work.
{% endif %}

## Design Specification

{{ specs }}

## Previous Phase Output

{% if previous_output %}
### Observation Phase Results
{{ previous_output }}
{% else %}
No previous phase output available.
{% endif %}

## Your Task

Implement the feature according to the design specification and observation phase findings.

Refer to the observation phase output for:
- Files to create
- Files to modify
- Implementation strategy

## Project Coding Standards

{{ coding_standards }}

## Implementation Steps

1. **Create New Files**
   - Create any new modules or files identified in observation phase
   - Set up proper module structure
   - Add necessary imports and dependencies

2. **Implement Core Logic**
   - Implement the main functionality
   - Follow the architecture from design spec
   - Use appropriate design patterns

3. **Add Error Handling**
   - Define custom error types if needed
   - Handle all error cases properly
   - Provide meaningful error messages

4. **Add Documentation**
   - Add doc comments for public APIs
   - Document complex algorithms
   - Add examples in doc comments where helpful

5. **Integrate with Existing Code**
   - Update existing files as needed
   - Ensure compatibility with existing APIs
   - Update module exports

## Output Requirements

After implementation, provide:

1. **Summary of Changes**
   - List all files created
   - List all files modified
   - Brief description of each change

2. **Implementation Notes**
   - Any deviations from design spec (with justification)
   - Any technical decisions made
   - Any concerns or issues encountered

3. **Next Steps**
   - What needs to be tested in Phase 3
   - Any edge cases to consider
   - Any integration points to verify

Begin implementation now. Use Write and Edit tools to create and modify files.
