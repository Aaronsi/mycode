# Phase 3: Write and Run Tests

## Feature: {{ feature_slug }}
**Repository**: {{ repo_path }}

{% if resume_info %}
## Resume Information
**Resuming from interrupted execution**
- Last completed phase: {{ resume_info.last_completed_phase }}
- Interrupted at: {{ resume_info.interrupted_at }}
- Reason: {{ resume_info.interrupt_reason }}
- Completed phases: {{ resume_info.completed_phases | join(", ") }}

Please continue testing from where you left off.
{% endif %}

## Design Specification

{{ specs }}

## Previous Phase Output

{% if previous_output %}
### Build Phase Results
{{ previous_output }}
{% else %}
No previous phase output available.
{% endif %}

## Your Task

Write comprehensive tests for the implemented feature and ensure all tests pass.

### Testing Requirements

1. **Unit Tests**
   - Test individual functions and methods in isolation
   - Test edge cases and error conditions
   - Test boundary conditions
   - Use descriptive test names: `test_should_[expected_behavior]`

2. **Integration Tests**
   - Test component interactions
   - Test end-to-end workflows
   - Test with realistic data
   - Place in `tests/` directory

3. **Test Coverage**
   - Cover all public APIs
   - Cover error paths
   - Cover edge cases
   - Aim for high coverage on critical paths

## Verification Criteria

From the verification document:
{{ verification_criteria }}

Ensure all verification criteria are covered by tests.

## Output Requirements

After testing, provide:

1. **Test Summary**
   - Number of unit tests added
   - Number of integration tests added
   - Test coverage summary
   - All tests passing confirmation

2. **Test Results**
   ```
   cargo test output:
   [paste output here]
   ```

3. **Clippy Results**
   ```
   cargo clippy output:
   [paste output here]
   ```

4. **Issues Found**
   - Any bugs discovered during testing
   - Any edge cases not handled
   - Any improvements needed

5. **Next Steps**
   - What needs verification in Phase 4
   - Any concerns for code review

Begin writing and running tests now.
