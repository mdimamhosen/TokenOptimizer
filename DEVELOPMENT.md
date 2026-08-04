# AI Assist Development Guide

Complete guide for developing, testing, and deploying AI Assist.

## Table of Contents

1. [Setup](#setup)
2. [Project Structure](#project-structure)
3. [Development Workflow](#development-workflow)
4. [Testing](#testing)
5. [Code Quality](#code-quality)
6. [Building](#building)
7. [Debugging](#debugging)
8. [Adding Features](#adding-features)

## Setup

### Prerequisites

- **Rust 1.75+** ([Install](https://rustup.rs/))
- **Git**
- **VS Code** (recommended) with Rust Analyzer

### Initial Setup

```bash
# Clone
git clone https://github.com/ai-assist-dev/ai-assist
cd ai-assist

# Verify Rust
rustc --version
cargo --version

# Update Rust
rustup update

# Install dev tools
cargo install cargo-watch cargo-tarpaulin cargo-audit

# Build all
cargo build --all

# Run tests
cargo test --all

# Check everything works
cargo run -p ai-assist-cli -- --help
```

### VS Code Setup

Create `.vscode/settings.json`:

```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.diagnostics.experimental.enable": true,
  "[rust]": {
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "files.watcherExclude": {
    "**/target": true
  }
}
```

## Project Structure

### Workspace Layout

```
ai-assist/
├── crates/
│   ├── core/                    # Core library
│   │   ├── src/
│   │   │   ├── lib.rs          # Root module
│   │   │   ├── error.rs        # Error types
│   │   │   ├── models.rs       # Data models
│   │   │   ├── detection.rs    # Tool detection
│   │   │   ├── database.rs     # Database ops
│   │   │   ├── token_counter.rs # Token counting
│   │   │   ├── optimization.rs  # Analysis
│   │   │   └── adapters/       # Tool adapters
│   │   └── Cargo.toml
│   │
│   ├── cli/                     # CLI application
│   │   ├── src/
│   │   │   └── main.rs         # CLI entry point
│   │   └── Cargo.toml
│   │
│   ├── desktop/                 # Tauri GUI
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── app.rs
│   │   └── Cargo.toml
│   │
│   └── adapters/                # Tool adapters
│       ├── src/
│       │   └── extractors.rs
│       └── Cargo.toml
│
├── docs/                        # Documentation
├── .github/                     # GitHub config
│   └── workflows/               # CI/CD pipelines
├── Cargo.toml                   # Workspace manifest
├── Cargo.lock
├── README.md
├── CONTRIBUTING.md
├── DEVELOPMENT.md              # This file
└── PROJECT_STATUS.md
```

### Module Responsibilities

| Module | Purpose | Key Types |
|--------|---------|-----------|
| `lib.rs` | Root, exports, config | `Config`, public APIs |
| `error.rs` | Error handling | `Error`, `Result` |
| `models.rs` | Data definitions | `AiTool`, `Session`, etc. |
| `detection.rs` | Tool detection | `ToolDetector`, results |
| `database.rs` | Storage operations | `Database`, queries |
| `token_counter.rs` | Token estimation | Token/cost calculations |
| `optimization.rs` | Analysis | `OptimizationEngine` |
| `adapters/` | Tool-specific | `ToolAdapter` trait |

## Development Workflow

### Daily Development

```bash
# Start watch mode (auto-rebuild on changes)
cargo watch -x build -x clippy

# In another terminal, run tests
cargo watch -x test

# Or specific test
cargo watch -x 'test --lib database'
```

### Making Changes

1. **Create feature branch**
   ```bash
   git checkout -b feature/my-feature
   ```

2. **Edit code**
   ```bash
   # Make changes in crates/
   # VS Code auto-formats on save
   ```

3. **Run checks locally**
   ```bash
   cargo fmt --all
   cargo clippy --all
   cargo test --all
   ```

4. **Commit**
   ```bash
   git add .
   git commit -m "feat: Add my feature"
   ```

5. **Push and create PR**
   ```bash
   git push origin feature/my-feature
   ```

### Code Review Checklist

- [ ] Builds with `cargo build --release`
- [ ] No warnings: `cargo clippy`
- [ ] Formatted: `cargo fmt`
- [ ] Tests pass: `cargo test`
- [ ] Docs generated: `cargo doc`
- [ ] No unsafe code (unless justified)
- [ ] Error handling complete
- [ ] Platform-specific code tested

## Testing

### Running Tests

```bash
# All tests
cargo test --all

# Specific crate
cargo test -p ai-assist-core

# Specific test
cargo test test_config_creation

# With output
cargo test -- --nocapture

# Specific test module
cargo test models::

# Ignore slow tests
cargo test -- --skip integration
```

### Writing Tests

In same file as code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let result = my_function();
        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert!(result.is_ok());
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("Expected panic");
    }
}
```

### Test Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --workspace

# With HTML report
cargo tarpaulin --workspace --out Html
```

### Integration Tests

Create `tests/` directory:

```rust
// tests/integration_tests.rs
use ai_assist_core::Config;

#[test]
fn test_full_workflow() {
    let config = Config::new().unwrap();
    // Test complete workflow
}
```

Run with:
```bash
cargo test --test integration_tests
```

## Code Quality

### Formatting

```bash
# Auto-format
cargo fmt --all

# Check without changing
cargo fmt --all -- --check
```

### Linting with Clippy

```bash
# Check all
cargo clippy --all

# Fix automatically
cargo clippy --all --fix

# With all warnings
cargo clippy --all -- -W clippy::all
```

### Security Audit

```bash
# Install audit
cargo install cargo-audit

# Check vulnerabilities
cargo audit
```

### Documentation

```bash
# Generate docs
cargo doc --no-deps --open

# Test doc examples
cargo test --doc

# Check docs build
cargo doc --no-deps --document-private-items
```

## Building

### Development Build

```bash
# Fast, with debug info
cargo build

# For specific crate
cargo build -p ai-assist-cli
```

### Release Build

```bash
# Optimized binary
cargo build --release

# Binary location
./target/release/ai-assist

# Stripped binary (smaller)
cargo build --release && strip target/release/ai-assist
```

### Build for Different Platforms

```bash
# List installed targets
rustup target list

# Add target
rustup target add x86_64-pc-windows-msvc

# Build for target
cargo build --release --target x86_64-pc-windows-msvc
```

### Build Sizes

```bash
# Check binary size
ls -lh target/release/ai-assist

# Reduce size
cargo build --release -Z build-std=std --target x86_64-unknown-linux-gnu
```

## Debugging

### Debug Output

```bash
# Enable debug logging
RUST_LOG=debug cargo run -p ai-assist-cli -- init

# Specific module
RUST_LOG=ai_assist_core=debug cargo run -p ai-assist-cli -- init

# With backtrace
RUST_BACKTRACE=1 cargo run -p ai-assist-cli -- init
RUST_BACKTRACE=full cargo run -p ai-assist-cli -- init
```

### Print Debugging

```rust
// In code
println!("Debug: {:?}", variable);
dbg!(variable);

// With tracing
use tracing::debug;
debug!("Custom debug message: {:?}", variable);
```

### Debug with GDB (Linux/macOS)

```bash
# Build with debug info
cargo build

# Run with gdb
gdb ./target/debug/ai-assist

# In gdb:
(gdb) run init
(gdb) break detection.rs:50
(gdb) continue
```

### VS Code Debugging

Install "CodeLLDB" extension, create `.vscode/launch.json`:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug CLI",
            "cargo": {
                "args": [
                    "build",
                    "-p", "ai-assist-cli",
                    "--bin", "ai-assist"
                ],
                "filter": {
                    "name": "ai-assist",
                    "kind": "bin"
                }
            },
            "args": ["init"],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

## Adding Features

### Adding a New Command

1. **Update enum** in `crates/cli/src/main.rs`:
```rust
#[derive(Subcommand)]
enum Commands {
    // ... existing
    MyNewCommand { #[arg] option: Option<String> },
}
```

2. **Add handler**:
```rust
Commands::MyNewCommand { option } => cmd_my_new_command(&config, option).await?,
```

3. **Implement handler**:
```rust
async fn cmd_my_new_command(config: &Config, option: Option<String>) -> anyhow::Result<()> {
    // Implementation
    println!("Running my new command");
    Ok(())
}
```

### Adding a New Model

1. **Add to** `crates/core/src/models.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MyModel {
    pub id: String,
    pub name: String,
}
```

2. **Update database** in `crates/core/src/database.rs`:
```rust
pub async fn save_my_model(&self, model: &MyModel) -> Result<()> {
    sqlx::query(
        "INSERT INTO my_models (id, name) VALUES (?, ?)"
    )
    .bind(&model.id)
    .bind(&model.name)
    .execute(&self.pool)
    .await?;
    Ok(())
}
```

### Adding a New Tool Adapter

1. **Update `AiTool` enum** in `models.rs`
2. **Add detection** in `detection.rs`
3. **Create adapter** in `crates/adapters/src/`
4. **Implement `ToolAdapter` trait**
5. **Add tests**

### Adding Database Table

1. **Add migration** in `database.rs` migrate() function:
```rust
sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS my_table (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        created_at TIMESTAMP NOT NULL
    )
    "#,
)
.execute(&self.pool)
.await?;
```

2. **Add model** in `models.rs`
3. **Add database operations** in `database.rs`
4. **Write tests**

## Performance Optimization

### Profiling

```bash
# CPU profiling (Linux)
cargo install flamegraph
cargo flamegraph -p ai-assist-cli --bin ai-assist

# Memory profiling
cargo install valgrind
valgrind ./target/release/ai-assist usage
```

### Optimization Techniques

1. **Database Queries**
   - Use indexes for frequent queries
   - Batch operations when possible
   - Use prepared statements

2. **Memory**
   - Avoid cloning large structures
   - Use references where possible
   - Stream large datasets

3. **Startup Time**
   - Lazy load modules
   - Cache results
   - Avoid unnecessary initialization

### Benchmark

```rust
#[bench]
fn bench_token_counting(b: &mut Bencher) {
    b.iter(|| TokenCounter::estimate_tokens("test text"))
}
```

Run with:
```bash
cargo bench
```

## Releases

### Creating a Release

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md**
3. **Create tag**: `git tag v0.2.0`
4. **Push**: `git push origin v0.2.0`
5. **GitHub Actions** builds and publishes automatically

### Version Numbering

Use Semantic Versioning:
- `0.1.0` - Initial release
- `0.2.0` - Features added
- `0.2.1` - Bug fix
- `1.0.0` - Stable/production

### Release Checklist

- [ ] All tests pass
- [ ] No warnings in build
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped
- [ ] Tag created
- [ ] Release notes written

## Repository Maintenance

### Keeping Updated

```bash
# Fetch latest
git fetch origin

# Update dependencies
cargo update

# Check for security issues
cargo audit

# Clean build artifacts
cargo clean
```

### Dependency Management

```bash
# Add dependency
cargo add serde --features derive

# Remove dependency
cargo remove serde_json

# Update specific package
cargo update -p tokio

# Check outdated
cargo outdated
```

## Documentation

### Writing Code Comments

```rust
/// Brief description
///
/// Longer explanation if complex
///
/// # Examples
/// ```
/// let result = function();
/// ```
///
/// # Panics
/// Panics if...
///
/// # Errors
/// Returns error if...
pub fn function() {}
```

### Update Docs

- Update README.md when features change
- Update ARCHITECTURE.md for design changes
- Update docs/ guides for new commands
- Add examples for new APIs

## Getting Help

```bash
# Rust documentation
rustup doc

# Cargo help
cargo --help
cargo build --help

# Crate documentation
cargo doc -p ai-assist-core --open

# Community
https://users.rust-lang.org/
https://stackoverflow.com/questions/tagged/rust
```

## Useful Shortcuts

| Command | Purpose |
|---------|---------|
| `cargo check` | Fast compile check |
| `cargo build -p <crate>` | Build specific crate |
| `cargo run -p <crate> -- <args>` | Run with arguments |
| `cargo test <filter>` | Run filtered tests |
| `cargo fix --allow-dirty` | Auto-fix issues |
| `cargo expand` | Show macro expansion |
| `cargo tree` | Show dependency tree |

## Performance Tips

1. Use `cargo check` instead of `cargo build` while developing
2. Use `cargo build --release` for benchmarking
3. Use `cargo watch` to auto-rebuild on changes
4. Use `cargo clippy` before committing
5. Enable LTO in release builds: Add to Cargo.toml:

```toml
[profile.release]
lto = true
codegen-units = 1
```

---

**Last Updated**: August 5, 2026  
**Maintained by**: AI Assist Team
