# Development Process

## pdf2md Development Standards and Workflow

### Overview
This document defines the development process, quality standards, and workflows for the pdf2md project. All contributors must follow these guidelines to ensure code quality, maintainability, and consistency.

---

## Development Methodology

### Test-Driven Development (TDD)

This project follows strict Test-Driven Development practices using the **Red-Green-Refactor** cycle.

#### The Red-Green-Refactor Cycle

```
+-------------+
|     RED     |  Write a failing test
|   (Test)    |
+------+------+
       |
       v
+-------------+
|    GREEN    |  Write minimal code to pass
|   (Code)    |
+------+------+
       |
       v
+-------------+
|  REFACTOR   |  Clean up and improve
|  (Improve)  |
+------+------+
       |
       +-------> Repeat
```

#### Step 1: RED - Write Failing Test
**Before writing any production code**, write a test that fails.

**Rules**:
- Write ONE test at a time
- The test should fail for the right reason (not compile error)
- Test should be specific and focused
- Name tests clearly: `test_<what>_<condition>_<expected>`

**Example**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input_path_with_nonexistent_file() {
        let path = Path::new("/nonexistent/file.pdf");
        let result = validate_input_path(path);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Pdf2MdError::InvalidInput(_)));
    }
}
```

**Verify RED**:
```bash
cargo test test_validate_input_path_with_nonexistent_file
# Should fail with expected error
```

#### Step 2: GREEN - Make Test Pass
Write **minimal code** to make the test pass. Don't write extra features.

**Rules**:
- Write only enough code to pass the current test
- Code doesn't need to be perfect yet
- Focus on making test pass, not on elegance
- Avoid premature optimization

**Example**:
```rust
fn validate_input_path(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(Pdf2MdError::InvalidInput(
            format!("Input file does not exist: {}", path.display())
        ));
    }
    Ok(())
}
```

**Verify GREEN**:
```bash
cargo test test_validate_input_path_with_nonexistent_file
# Should pass
```

#### Step 3: REFACTOR - Improve Code
Now improve the code while keeping tests green.

**Rules**:
- Improve code quality without changing behavior
- Add documentation
- Extract helper functions
- Improve variable names
- Optimize if needed
- **Tests must still pass after refactoring**

**Example**:
```rust
/// Validate that the input path exists and is a file
///
/// # Arguments
/// * `path` - Path to validate
///
/// # Errors
/// Returns `InvalidInput` if path doesn't exist or isn't a file
fn validate_input_path(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(Pdf2MdError::InvalidInput(
            format!("Input file does not exist: {}", path.display())
        ));
    }

    if !path.is_file() {
        return Err(Pdf2MdError::InvalidInput(
            format!("Input path is not a file: {}", path.display())
        ));
    }

    Ok(())
}
```

**Verify REFACTOR**:
```bash
cargo test
# All tests should still pass
```

#### TDD Best Practices

**DO**:
- Write tests first, always
- Keep tests simple and focused
- Test one thing per test
- Use descriptive test names
- Run tests frequently
- Commit after each red-green-refactor cycle

**DON'T**:
- Skip writing tests
- Write tests after code
- Write multiple tests at once
- Write code without a failing test
- Ignore failing tests
- Write overly complex tests

---

## Modular Design Principles

### Keep Functions Small
- Each function should do ONE thing
- Ideal: < 20 lines
- Maximum: < 50 lines
- If longer, break into helper functions

### Single Responsibility Principle
Each module, struct, and function should have one clear purpose.

**Example**:
```rust
// GOOD: Single responsibility
fn validate_input_path(path: &Path) -> Result<()> { ... }
fn validate_output_path(path: &Path) -> Result<()> { ... }

// BAD: Multiple responsibilities
fn validate_paths(input: &Path, output: &Path) -> Result<()> { ... }
```

### Documentation Requirements
All public APIs must have documentation comments.

**Required Elements**:
```rust
/// Brief one-line description
///
/// Optional longer description with details.
///
/// # Arguments
/// * `param1` - Description of param1
/// * `param2` - Description of param2
///
/// # Returns
/// Description of return value
///
/// # Errors
/// When this function returns errors
///
/// # Examples
/// ```
/// let result = my_function(arg1, arg2);
/// ```
pub fn my_function(param1: Type1, param2: Type2) -> Result<ReturnType> {
    // ...
}
```

### Module Organization
```
src/
+-- lib.rs          # Public API, orchestration
+-- main.rs         # CLI entry point (minimal)
+-- cli.rs          # Argument parsing
+-- config.rs       # Configuration
+-- error.rs        # Error types
+-- pdf.rs          # PDF processing
+-- markdown.rs     # Markdown generation
```

**Rules**:
- Keep main.rs minimal (< 30 lines)
- Put all logic in lib.rs and modules
- Each module in separate file
- Use clear module boundaries

---

## Quality Checklist

### Pre-Commit Quality Checklist

**BEFORE EVERY COMMIT**, run through this checklist:

#### 1. Formatting
```bash
cargo fmt
```
- [ ] Code is formatted according to rustfmt
- [ ] No manual formatting adjustments needed

#### 2. Linting
```bash
cargo clippy -- -D warnings
```
- [ ] Zero clippy warnings
- [ ] All clippy suggestions addressed
- [ ] **NEVER use #[allow(clippy::...)]** - fix issues instead
- [ ] **NEVER disable or ignore warnings**

#### 3. Build
```bash
cargo build --release
```
- [ ] Project builds successfully
- [ ] Zero compiler warnings
- [ ] Zero compiler errors

#### 4. Testing
```bash
cargo test
```
- [ ] All tests pass
- [ ] No ignored tests (unless documented why)
- [ ] New functionality has tests
- [ ] Tests are meaningful (not just placeholders)

#### 5. Documentation
```bash
cargo doc --no-deps --open
```
- [ ] All public APIs documented
- [ ] Documentation is accurate
- [ ] Examples compile (if present)
- [ ] docs/ files updated if needed

#### 6. Dependencies
- [ ] Only necessary dependencies added
- [ ] No unused dependencies
- [ ] Dependency versions pinned appropriately

#### 7. Git
- [ ] .gitignore is appropriate
- [ ] No sensitive data committed
- [ ] No build artifacts committed
- [ ] Commit message is clear and descriptive

### Quality Standards

#### Rust Edition
- **Must use**: Rust 2024 edition
- Specified in `Cargo.toml`: `edition = "2024"`

#### Warnings Policy
- **Zero tolerance for warnings**
- All build warnings must be fixed
- All clippy warnings must be fixed
- **Never disable warnings** with:
  - `#[allow(dead_code)]`
  - `#[allow(unused_variables)]`
  - `#[allow(clippy::...)]`
  - etc.

**If clippy suggests something**:
1. Understand the suggestion
2. Fix the issue properly
3. If genuinely not applicable, reconsider design
4. Document why in code review

#### Code Coverage
- **Minimum**: 80% code coverage
- **Goal**: 90%+ code coverage
- Critical paths: 100% coverage

**Check coverage** (requires cargo-tarpaulin):
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

---

## Git Workflow

### Commit Guidelines

#### Commit Frequency
- Commit after each red-green-refactor cycle
- Commit when a feature is complete
- Commit before making major changes

#### Commit Message Format
```
<type>: <short summary>

<optional longer description>

<optional footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `test`: Adding tests
- `refactor`: Code refactoring
- `chore`: Maintenance tasks

**Examples**:
```
feat: add PDF validation function

Implement validate_pdf() to check PDF magic bytes.
Includes tests for valid and invalid PDFs.

test: add integration tests for CLI help flag

refactor: extract path validation to helper function

Split validate_input_path into smaller functions for clarity.
```

#### Commit Best Practices
- Keep commits focused and atomic
- Don't commit broken code
- Don't commit commented-out code
- Don't commit debug print statements
- Run quality checklist before committing

### Branching Strategy
- `main` branch: Always stable and deployable
- Feature branches: For new features
- Work can be done directly on main for small projects

---

## Testing Standards

### Test Organization

#### Unit Tests
Located in same file as code:
```rust
// src/config.rs

pub fn validate_input_path(path: &Path) -> Result<()> {
    // Implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input_path_with_valid_file() {
        // Test code
    }

    #[test]
    fn test_validate_input_path_with_nonexistent_file() {
        // Test code
    }
}
```

#### Integration Tests
Located in `tests/` directory:
```rust
// tests/integration_test.rs

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("pdf2md").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("PDF to Markdown"));
}
```

### Test Fixtures
- Store test files in `tests/fixtures/`
- Include various PDF types
- Keep fixtures small (< 1MB)
- Document what each fixture tests

### Test Naming Convention
```rust
#[test]
fn test_<function_name>_<scenario>_<expected_result>() {
    // Format: test_what_when_then
}
```

**Examples**:
- `test_validate_pdf_with_valid_file_returns_ok()`
- `test_extract_text_with_empty_pdf_returns_empty_string()`
- `test_format_content_with_special_chars_escapes_correctly()`

### Test Coverage Requirements

**Must Test**:
- Happy path (normal operation)
- Error cases
- Edge cases (empty input, very large input, etc.)
- Boundary conditions

**Testing Strategies**:
- Use `assert!()` for boolean conditions
- Use `assert_eq!()` for equality
- Use `assert!(matches!(...))` for enum variants
- Use `Result<T, E>` assertions for error handling

---

## Code Review Guidelines

### Self-Review Checklist

Before considering code complete, review:

- [ ] Code follows TDD process
- [ ] All functions are small and focused
- [ ] All public APIs are documented
- [ ] Tests are comprehensive
- [ ] Error handling is robust
- [ ] No clippy warnings
- [ ] No compiler warnings
- [ ] Code is formatted
- [ ] Variable names are descriptive
- [ ] No premature optimization
- [ ] No unused imports or code
- [ ] Commit messages are clear

---

## Continuous Quality

### Regular Maintenance

**Daily**:
- Run tests before committing
- Address any warnings immediately

**Weekly**:
- Review code coverage
- Update dependencies if needed
- Review and update documentation

**Before Release**:
- Full quality audit
- Performance testing
- Documentation review
- README accuracy check

### Dependency Management

**Adding Dependencies**:
1. Evaluate necessity
2. Check maintenance status
3. Review security advisories
4. Use specific version numbers
5. Document why dependency is needed

**Updating Dependencies**:
```bash
cargo update
cargo test  # Ensure tests still pass
```

---

## Performance Considerations

### Performance Testing
- Profile with `cargo flamegraph` or similar
- Benchmark critical paths
- Test with large inputs
- Monitor memory usage

### Optimization Strategy
1. **First**: Make it work (correctness)
2. **Second**: Make it right (clean code)
3. **Third**: Make it fast (only if needed)

**Don't**:
- Optimize prematurely
- Sacrifice clarity for minor gains
- Skip tests for performance

---

## Security Considerations

### Input Validation
- Validate all user inputs
- Sanitize file paths
- Check file sizes
- Prevent path traversal

### Error Messages
- Don't expose internal details
- Don't leak file system structure
- Provide helpful but safe messages

---

## Documentation Standards

### README.md
Must include:
- Project description
- Use cases
- Installation instructions
- Usage examples
- Link to detailed docs
- License information

### Code Documentation
- Module-level documentation
- Public API documentation
- Complex algorithms explained
- Non-obvious behavior documented

### Documentation Files
Keep updated:
- architecture.md
- prd.md
- design.md
- plan.md
- process.md (this file)

---

## Summary

### Core Principles
1. **Test-Driven Development**: Always write tests first
2. **Modular Design**: Small, focused functions
3. **Quality First**: Zero warnings, comprehensive tests
4. **Documentation**: All public APIs documented
5. **Rust 2024**: Use latest edition features

### Quality Checklist (Quick Reference)
```bash
# Before every commit:
cargo fmt                           # Format code
cargo clippy -- -D warnings        # Lint (no warnings!)
cargo build --release              # Build (no warnings!)
cargo test                         # Test (all pass!)
cargo doc --no-deps               # Verify docs
# Review changes and commit
```

### Success Criteria
- All tests pass (100%)
- Zero compiler warnings
- Zero clippy warnings
- Documentation complete
- Code coverage > 80%
- Git history clean

---

## Questions?

If you have questions about the development process:
1. Check this document first
2. Review existing code for examples
3. Consult Rust documentation
4. Ask for clarification before proceeding

**Remember**: Quality over speed. Take time to do it right.
