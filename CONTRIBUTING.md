# Contributing to AI Assist

Thank you for your interest in contributing! This document provides guidelines and instructions.

## Code of Conduct

- Be respectful and inclusive
- Focus on the code, not the person
- Help others learn and grow
- Report issues constructively

## Getting Started

### Prerequisites

- Rust 1.75+ ([Install Rust](https://rustup.rs/))
- Git
- Your favorite code editor

### Setup Development Environment

```bash
# Clone repository
git clone https://github.com/ai-assist-dev/ai-assist
cd ai-assist

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run the CLI
cargo run -p ai-assist-cli -- --help
```

### Project Structure

```
crates/
├── core/          # Core library - no CLI dependencies
├── cli/           # CLI application
├── desktop/       # Tauri GUI (planned)
├── adapters/      # Tool-specific implementations
```

## Development Workflow

### Creating a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### Code Style

- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Prefer explicit over implicit
- Write self-documenting code

### Running Quality Checks

```bash
# Format code
cargo fmt --all

# Lint code
cargo clippy --all --all-targets -- -D warnings

# Run tests
cargo test --workspace

# Build release binary
cargo build --release --workspace
```

### Writing Tests

Tests go in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        let result = my_function();
        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn test_async_feature() {
        let result = my_async_function().await;
        assert!(result.is_ok());
    }
}
```

Run tests:

```bash
cargo test --workspace -- --nocapture
```

## Areas for Contribution

### High Priority

1. **Adapter Implementations**
   - Complete data extraction for each tool
   - Handle tool-specific log formats
   - Parse configuration files

2. **Real-time Monitoring**
   - Watch current session metrics
   - Stream token usage updates
   - Display live cost tracking

3. **Desktop Dashboard**
   - Tauri application setup
   - React components for visualization
   - Real-time chart updates

### Medium Priority

1. **Token Counting**
   - Implement accurate tokenizers
   - Handle edge cases
   - Support more models

2. **Optimization Engine**
   - Add more detection rules
   - Improve scoring algorithm
   - Generate better recommendations

3. **Platform Optimization**
   - Performance improvements
   - Memory usage reduction
   - Faster tool detection

### Good First Issues

- Documentation improvements
- Test coverage increases
- Bug fixes
- UI/UX enhancements
- Platform-specific issues

## Submitting Changes

### Create a Pull Request

1. Push to your feature branch
2. Open a PR with descriptive title
3. Reference any related issues
4. Ensure CI passes

### PR Template

```markdown
## Description
Brief explanation of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation
- [ ] Performance
- [ ] Refactoring

## Related Issues
Fixes #issue-number

## Testing
Describe how you tested changes

## Checklist
- [ ] Code follows style guidelines
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] No breaking changes
```

### Review Process

1. At least one maintainer review required
2. CI checks must pass
3. Code coverage maintained
4. No unresolved comments

## Commit Messages

Use clear, descriptive commit messages:

```
feat: Add real-time monitoring command

- Implement watch command with --interval flag
- Add MonitoringState model
- Update CLI help documentation

Fixes #123
```

Format:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `refactor:` - Code refactoring
- `test:` - Test additions
- `perf:` - Performance improvement

## Adding Support for a New AI Tool

### Step 1: Update Models

In `crates/core/src/models.rs`:

```rust
pub enum AiTool {
    // ... existing tools
    MyNewTool,  // Add here
}

impl AiTool {
    pub fn as_str(&self) -> &'static str {
        match self {
            // ... existing matches
            AiTool::MyNewTool => "my_new_tool",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            // ... existing matches
            AiTool::MyNewTool => "My New Tool",
        }
    }
}
```

### Step 2: Implement Detection

In `crates/core/src/detection.rs`:

```rust
async fn detect_my_new_tool(&self) -> Result<(Option<String>, Option<String>, Option<String>)> {
    let tool_dir = self.home_dir.join(".my-tool");
    if tool_dir.exists() {
        return Ok((
            Some(tool_dir.to_string_lossy().into_owned()),
            None,  // version if available
            Some(tool_dir.to_string_lossy().into_owned()),  // config path
        ));
    }
    Ok((None, None, None))
}
```

Add to detection list:

```rust
let tools = vec![
    // ... existing tools
    (AiTool::MyNewTool, "my-tool"),
];
```

### Step 3: Create Adapter

In `crates/adapters/src/`:

```rust
pub struct MyNewToolAdapter;

impl ToolAdapter for MyNewToolAdapter {
    fn tool(&self) -> AiTool {
        AiTool::MyNewTool
    }

    fn parse_sessions(&self, tool_path: &Path) -> Result<Vec<Session>> {
        // Parse tool-specific session data
        Ok(Vec::new())
    }

    fn get_config_files(&self, tool_path: &Path) -> Result<Vec<String>> {
        // Return relevant configuration files
        Ok(Vec::new())
    }
}
```

### Step 4: Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let detector = ToolDetector::new(PathBuf::from("/tmp"));
        // Test detection logic
    }
}
```

## Documentation

### Code Documentation

```rust
/// Brief description
///
/// Longer explanation if needed
///
/// # Examples
/// ```
/// let result = function();
/// ```
pub fn function() {}
```

### README Sections

Update README.md when:
- Adding commands
- Changing installation
- Adding features
- Breaking changes

### ARCHITECTURE.md

Update when:
- Changing data model
- Adding new modules
- Modifying key algorithms

## Reporting Bugs

### Issue Template

```markdown
## Description
What is the problem?

## Steps to Reproduce
1. First step
2. Second step
3. Expected behavior
4. Actual behavior

## Environment
- OS: Windows 10 / macOS 12 / Ubuntu 22.04
- Rust version: 1.75
- AI Assist version: 0.1.0

## Logs
```
Relevant error messages
```
```

## Performance Improvements

When submitting performance improvements:

1. Provide benchmark results
2. Explain the improvement
3. Show before/after metrics
4. Ensure no correctness regression

## Review Feedback

When receiving feedback:

1. Thank the reviewer
2. Ask clarifying questions if needed
3. Address comments
4. Re-request review when done

## Questions?

- Check [GitHub Issues](https://github.com/ai-assist-dev/ai-assist/issues)
- Open a [GitHub Discussion](https://github.com/ai-assist-dev/ai-assist/discussions)
- See [ARCHITECTURE.md](ARCHITECTURE.md) for design details

## License

By contributing, you agree your contributions are licensed under MIT License.

---

Thank you for making AI Assist better! 🚀
