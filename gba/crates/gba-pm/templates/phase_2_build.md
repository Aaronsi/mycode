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

### Implementation Requirements

1. **Follow Design Specification**
   - Implement exactly as specified in the design document
   - Follow the file change plan from observation phase
   - Maintain consistency with existing codebase patterns

2. **Code Quality Standards**
   - Write clean, idiomatic Rust code
   - Follow the project's coding standards (see below)
   - Add appropriate error handling (no unwrap/expect in production code)
   - Include inline documentation for complex logic
   - Use meaningful variable and function names

3. **Error Handling**
   - Use `Result<T>` for fallible operations
   - Use `thiserror` for library error types
   - Use `anyhow` for application error handling
   - Provide context with `.context()` when propagating errors

4. **Type Safety**
   - Make types as specific as possible
   - Use enums for state machines
   - Implement `Debug` for all types
   - Use builder pattern for complex constructors (typed-builder crate)

5. **Async Code**
   - Use tokio for async runtime
   - Prefer message passing over shared state
   - Use proper async traits (native or async-trait when needed)
   - Handle task panics appropriately

## Project Coding Standards

{{ coding_standards }}

## Previous Phase Findings

The observation phase identified the files and changes needed. Refer to the previous phase output for:
- Files to create
- Files to modify
- Implementation strategy

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

## Important Notes

- **DO NOT** write tests in this phase (tests come in Phase 3)
- **DO NOT** run the code yet (testing comes in Phase 3)
- **DO** focus on implementation only
- **DO** follow existing code patterns and conventions
- **DO** ask for clarification if design spec is ambiguous

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
