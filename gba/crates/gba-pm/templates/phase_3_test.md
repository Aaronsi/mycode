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

4. **Test Organization**
   - Unit tests in same file: `#[cfg(test)] mod tests`
   - Integration tests in `tests/` directory
   - Use `rstest` for parameterized tests if needed
   - Use descriptive module names

### Testing Steps

1. **Write Unit Tests**
   - Add `#[cfg(test)]` module to each implementation file
   - Test each public function
   - Test error cases with `assert!(matches!(...))` or `assert_eq!`
   - Use `#[should_panic]` for panic tests

2. **Write Integration Tests**
   - Create test files in `tests/` directory
   - Test realistic scenarios
   - Test component interactions
   - Mock external dependencies if needed

3. **Run Tests**
   - Run `cargo test` to execute all tests
   - Fix any failing tests
   - Ensure all tests pass

4. **Check Code Quality**
   - Run `cargo clippy -- -D warnings`
   - Fix all clippy warnings
   - Run `cargo fmt` to format code
   - Ensure code compiles without warnings

5. **Verify Functionality**
   - Manually test critical paths if needed
   - Verify error messages are clear
   - Check edge cases

## Test Examples

### Unit Test Example
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_return_success_for_valid_input() {
        let result = my_function("valid");
        assert!(result.is_ok());
    }

    #[test]
    fn test_should_return_error_for_invalid_input() {
        let result = my_function("");
        assert!(matches!(result, Err(MyError::InvalidInput)));
    }
}
```

### Integration Test Example
```rust
// tests/integration_test.rs
use my_crate::*;

#[test]
fn test_should_complete_full_workflow() {
    // Setup
    let config = Config::default();

    // Execute
    let result = run_workflow(config);

    // Verify
    assert!(result.is_ok());
    assert_eq!(result.unwrap().status, Status::Completed);
}
```

## Verification Criteria

From the verification document:
{{ verification_criteria }}

Ensure all verification criteria are covered by tests.

## Important Notes

- **DO** write comprehensive tests
- **DO** run `cargo test` and ensure all tests pass
- **DO** run `cargo clippy` and fix all warnings
- **DO** test error cases explicitly
- **DO NOT** skip tests for "obvious" functionality
- **DO NOT** leave failing tests

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

Begin writing and running tests now. Use Write/Edit tools to add tests, and Bash tool to run `cargo test` and `cargo clippy`.
