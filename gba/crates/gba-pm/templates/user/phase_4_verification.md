# Phase 4: Verification

## Feature: {{ feature_slug }}
**Repository**: {{ repo_path }}

{% if resume_info %}
## Resume Information
**Resuming from interrupted execution**
- Last completed phase: {{ resume_info.last_completed_phase }}
- Interrupted at: {{ resume_info.interrupted_at }}
- Reason: {{ resume_info.interrupt_reason }}
- Completed phases: {{ resume_info.completed_phases | join(", ") }}

Please continue verification from where you left off.
{% endif %}

## Design Specification

{{ specs }}

## Verification Criteria

{{ verification_criteria }}

## Previous Phase Output

{% if previous_output %}
### Test Phase Results
{{ previous_output }}
{% else %}
No previous phase output available.
{% endif %}

## Your Task

Verify that the implementation meets all requirements and acceptance criteria defined in the design specification.

### Verification Steps

1. **Review Functional Requirements**
   - Go through each requirement in verification.md
   - Test each requirement manually or verify test coverage
   - Mark each requirement as ✅ verified or ❌ not met

2. **Review Non-Functional Requirements**
   - Check performance (run benchmarks if specified)
   - Review security implications
   - Test error handling thoroughly
   - Verify logging and observability

3. **Code Quality Check**
   - Run `cargo clippy -- -D warnings -W clippy::pedantic`
   - Run `cargo fmt --check`
   - Check for any TODO or FIXME comments
   - Verify all public APIs have documentation

4. **Integration Verification**
   - Test integration with existing code
   - Verify no breaking changes (unless intended)
   - Check backward compatibility if required
   - Test with realistic data and scenarios

5. **Documentation Verification**
   - Verify README is updated if needed
   - Check API documentation is complete
   - Verify examples work correctly
   - Check architecture docs are updated

## Testing Commands

```bash
# Run all tests
cargo test

# Run clippy with strict checks
cargo clippy -- -D warnings -W clippy::pedantic

# Check formatting
cargo fmt --check

# Build in release mode
cargo build --release
```

## Output Requirements

After verification, provide:

1. **Verification Report** with status of each requirement
2. **Test Results** from cargo test and clippy
3. **Issues and Recommendations**
4. **Sign-off**: ✅ verified or ❌ issues found

Begin verification now.
