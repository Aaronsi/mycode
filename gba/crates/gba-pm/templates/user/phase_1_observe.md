# Phase 1: Observe and Understand

## Feature: {{ feature_slug }}
**Repository**: {{ repo_path }}

{% if resume_info %}
## Resume Information
**Resuming from interrupted execution**
- Last completed phase: {{ resume_info.last_completed_phase }}
- Interrupted at: {{ resume_info.interrupted_at }}
- Reason: {{ resume_info.interrupt_reason }}

Please continue from where you left off.
{% endif %}

## Design Specification

{{ specs }}

## Your Task

Thoroughly understand the codebase and identify exactly what needs to be changed to implement this feature.

### Objectives

1. **Understand Existing Architecture**
   - Identify relevant modules, components, and their relationships
   - Understand data flow and control flow
   - Identify existing patterns and conventions

2. **Identify Impact Areas**
   - Which files need to be created?
   - Which files need to be modified?
   - What are the dependencies between changes?

3. **Assess Risks and Challenges**
   - What are potential breaking changes?
   - What are the technical challenges?
   - What are the edge cases to consider?

4. **Plan File Changes**
   - List all files to create with their purpose
   - List all files to modify with specific changes needed
   - Identify any files that need to be deleted or renamed

## Repository Context

### README
{{ readme }}

### Coding Standards
{{ coding_standards }}

## Previous Phase Output

{% if previous_output %}
{{ previous_output }}
{% else %}
This is the first phase. No previous output available.
{% endif %}

## Output Requirements

Provide a detailed analysis including:

1. **Architecture Summary**
   - Current relevant architecture
   - How the new feature fits in

2. **File Change Plan**
   ```
   Files to Create:
   - path/to/new_file.rs: [purpose]

   Files to Modify:
   - path/to/existing_file.rs: [changes needed]

   Files to Delete:
   - path/to/old_file.rs: [reason]
   ```

3. **Implementation Strategy**
   - Step-by-step approach
   - Order of changes
   - Testing strategy

4. **Risks and Mitigations**
   - Potential issues
   - How to address them

5. **Questions or Concerns**
   - Any ambiguities in requirements
   - Technical decisions needed

Begin your observation and analysis now.
