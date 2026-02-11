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

### Review Objectives

1. **Code Quality Review**
   - Check code readability and maintainability
   - Verify naming conventions are followed
   - Ensure code is well-structured and organized
   - Check for code duplication (DRY principle)

2. **Architecture Review**
   - Verify architecture matches design specification
   - Check separation of concerns
   - Validate abstraction levels
   - Ensure SOLID principles are followed

3. **Error Handling Review**
   - Verify all errors are handled properly
   - Check error messages are clear and actionable
   - Ensure no unwrap/expect in production code
   - Validate error types are appropriate

4. **Performance Review**
   - Check for unnecessary allocations
   - Verify efficient algorithms are used
   - Look for potential bottlenecks
   - Check async code is properly structured

5. **Security Review**
   - Check for potential security vulnerabilities
   - Verify input validation
   - Check for SQL injection, XSS, etc. (if applicable)
   - Ensure sensitive data is handled properly

6. **Documentation Review**
   - Verify all public APIs are documented
   - Check doc comments are clear and accurate
   - Ensure examples are provided where helpful
   - Verify inline comments explain complex logic

### Review Checklist

#### Code Quality
- [ ] Code is readable and self-documenting
- [ ] Naming is clear and consistent
- [ ] Functions are small and focused (< 150 lines)
- [ ] No code duplication
- [ ] Proper use of Rust idioms
- [ ] No unnecessary complexity

#### Architecture
- [ ] Follows design specification
- [ ] Proper separation of concerns
- [ ] Appropriate abstraction levels
- [ ] SOLID principles followed
- [ ] Consistent with existing codebase patterns

#### Error Handling
- [ ] All errors handled with Result<T>
- [ ] No unwrap/expect in production code
- [ ] Error messages are clear
- [ ] Proper error context provided
- [ ] Custom error types where appropriate

#### Performance
- [ ] No unnecessary allocations
- [ ] Efficient algorithms used
- [ ] Proper use of iterators
- [ ] Async code properly structured
- [ ] No blocking operations in async contexts

#### Security
- [ ] Input validation in place
- [ ] No SQL injection vulnerabilities
- [ ] No XSS vulnerabilities
- [ ] Sensitive data handled securely
- [ ] No hardcoded secrets

#### Documentation
- [ ] All public APIs documented
- [ ] Doc comments are clear
- [ ] Examples provided where helpful
- [ ] Complex logic explained
- [ ] Module-level documentation present

#### Testing
- [ ] Comprehensive test coverage
- [ ] Tests are clear and maintainable
- [ ] Edge cases covered
- [ ] Error cases tested

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

## Important Notes

- **DO** be thorough and critical
- **DO** fix issues found during review
- **DO** improve code clarity and maintainability
- **DO** ensure consistency with existing codebase
- **DO NOT** make unnecessary changes
- **DO NOT** introduce new features (scope creep)
- **DO NOT** skip re-testing after changes

## Output Requirements

After code review and refinement, provide:

1. **Review Summary**
   ```markdown
   # Code Review Report: {{ feature_slug }}

   ## Issues Found and Fixed

   ### Critical Issues
   - Issue 1: [description]
     - Location: [file:line]
     - Fix: [what was done]

   ### Major Issues
   - Issue 1: [description]
     - Location: [file:line]
     - Fix: [what was done]

   ### Minor Issues
   - Issue 1: [description]
     - Location: [file:line]
     - Fix: [what was done]

   ## Improvements Made
   - Improvement 1: [description]
   - Improvement 2: [description]

   ## Code Quality Metrics
   - Lines of code: [number]
   - Test coverage: [percentage]
   - Clippy warnings: 0
   - Documentation coverage: [percentage]

   ## Overall Assessment
   ✅ Code meets quality standards and is ready for PR
   ```

2. **Changes Made**
   - List all files modified during review
   - Describe changes made
   - Explain rationale for changes

3. **Final Verification**
   - Confirm all tests still pass
   - Confirm no clippy warnings
   - Confirm code is formatted

4. **Sign-off**
   - "✅ Code review complete. Implementation is ready for PR."

Begin code review now. Use Read tool to review files, Edit tool to make improvements, and Bash tool to verify changes.
