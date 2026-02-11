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

### Verification Objectives

1. **Functional Requirements Verification**
   - Verify each functional requirement is implemented
   - Test against acceptance criteria
   - Validate behavior matches specification

2. **Non-Functional Requirements Verification**
   - Performance: Check if performance requirements are met
   - Security: Verify security considerations are addressed
   - Reliability: Test error handling and edge cases
   - Maintainability: Check code quality and documentation

3. **Design Compliance**
   - Verify implementation follows design specification
   - Check architecture matches design
   - Validate API interfaces match specification

4. **Quality Assurance**
   - All tests pass
   - No clippy warnings
   - Code is properly formatted
   - Documentation is complete

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

### Verification Checklist

Go through each item in the verification criteria and mark as complete:

#### Functional Requirements
{{ verification_criteria }}

#### Non-Functional Requirements
- [ ] Performance: Meets performance requirements
- [ ] Security: No security vulnerabilities introduced
- [ ] Error Handling: All errors handled properly
- [ ] Logging: Appropriate logging in place
- [ ] Documentation: Complete and accurate

#### Code Quality
- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No unwrap/expect in production code
- [ ] All public APIs documented

#### Integration
- [ ] Integrates correctly with existing code
- [ ] No breaking changes (or documented if intentional)
- [ ] Backward compatible (if required)
- [ ] Works with realistic data

## Testing Commands

Run these commands to verify:

```bash
# Run all tests
cargo test

# Run clippy with strict checks
cargo clippy -- -D warnings -W clippy::pedantic

# Check formatting
cargo fmt --check

# Build in release mode
cargo build --release

# Run specific integration tests
cargo test --test integration_test_name
```

## Important Notes

- **DO** verify every requirement in verification.md
- **DO** test edge cases and error conditions
- **DO** check for security issues
- **DO** verify documentation is complete
- **DO NOT** skip verification steps
- **DO NOT** proceed if critical requirements are not met

## Output Requirements

After verification, provide:

1. **Verification Report**
   ```markdown
   # Verification Report: {{ feature_slug }}

   ## Functional Requirements
   - ✅ Requirement 1: [description] - Verified
   - ✅ Requirement 2: [description] - Verified
   - ❌ Requirement 3: [description] - Not met: [reason]

   ## Non-Functional Requirements
   - ✅ Performance: [results]
   - ✅ Security: [findings]
   - ✅ Error Handling: [verified]

   ## Code Quality
   - ✅ All tests pass (X tests, Y assertions)
   - ✅ No clippy warnings
   - ✅ Code formatted
   - ✅ Documentation complete

   ## Issues Found
   [List any issues discovered]

   ## Overall Status
   ✅ PASSED / ❌ FAILED
   ```

2. **Test Results**
   - Paste output of `cargo test`
   - Paste output of `cargo clippy`

3. **Issues and Recommendations**
   - Any issues that need to be fixed
   - Recommendations for improvement
   - Any concerns for code review

4. **Sign-off**
   - If all requirements met: "✅ Implementation verified and ready for review"
   - If issues found: "❌ Issues found that need to be addressed: [list]"

Begin verification now. Use Read, Bash, and other tools to thoroughly verify the implementation.
