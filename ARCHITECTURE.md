# AI Assist Architecture

## Overview

AI Assist is a modular, cross-platform developer analytics tool built with Rust. It provides universal token usage tracking and optimization insights for multiple AI coding assistants.

## Core Design Principles

1. **Privacy First** - All data stays local
2. **Multi-Tool Support** - Agnostic to which AI assistant you use
3. **Performance** - Native Rust for minimal overhead
4. **Modularity** - Clean separation of concerns
5. **Extensibility** - Easy to add new adapters

## Project Structure

```
ai-assist/
├── crates/
│   ├── core/           # Core engine (detection, analytics, database)
│   ├── cli/            # Command-line interface
│   ├── desktop/        # Tauri-based GUI (planned)
│   ├── adapters/       # Tool-specific adapters
│   └── npm/            # npm package wrapper (planned)
├── docs/               # Documentation
├── tests/              # Integration tests
└── Cargo.toml          # Workspace configuration
```

## Crate Overview

### Core (`ai-assist-core`)

The heart of AI Assist. Provides all core functionality:

#### Modules

- **detection** - Scans system for installed AI tools
  - `ToolDetector` - Detects tools and their configurations
  - Platform-specific path resolution (Windows, macOS, Linux)

- **database** - Local SQLite storage
  - `Database` - Connection management and migrations
  - Tables: tools, sessions, token_usage, project_analysis
  - Proper indexing for performance

- **models** - Type definitions
  - `AiTool` - Enum of supported tools
  - `DetectedTool` - Tool information
  - `Session` - Usage session
  - `TokenUsage` - Token records
  - `Recommendation` - Optimization suggestions

- **token_counter** - Token estimation and cost calculation
  - Supports Anthropic, OpenAI, and generic models
  - Cost estimation in USD

- **optimization** - Analysis engine
  - `OptimizationEngine` - Detects workflow inefficiencies
  - AI Readiness scoring
  - Configuration analysis

- **adapters** - Tool-specific implementations
  - Unified interface for extracting tool data
  - Implementations for each supported tool

### CLI (`ai-assist-cli`)

Command-line interface with the following commands:

```
ai-assist init        # Initialize and detect tools
ai-assist scan        # Scan and record tools
ai-assist usage       # Show token usage
ai-assist analyze     # Analyze project
ai-assist watch       # Real-time monitoring
ai-assist report      # Generate reports
ai-assist dashboard   # Open GUI dashboard
```

### Desktop (`ai-assist-desktop`)

Tauri-based graphical dashboard for:
- Interactive visualizations
- Real-time monitoring
- Detailed reports
- Settings management

*Currently in planning phase*

### Adapters (`ai-assist-adapters`)

Tool-specific data extractors:

Each adapter implements:
- Session data parsing
- Configuration file analysis
- Custom metrics extraction

Supported tools:
- Claude Code
- Cursor
- GitHub Copilot
- Windsurf
- Cline
- Continue
- Aider
- OpenAI Codex

## Data Model

### Database Schema

#### detected_tools
```sql
- id (TEXT PRIMARY KEY)
- tool (TEXT UNIQUE)
- installed (BOOLEAN)
- path (TEXT)
- version (TEXT)
- config_path (TEXT)
- last_detected (TIMESTAMP)
```

#### sessions
```sql
- id (TEXT PRIMARY KEY)
- tool (TEXT) -> detected_tools.tool
- project (TEXT)
- started_at (TIMESTAMP)
- ended_at (TIMESTAMP)
- tokens_used (INTEGER)
- estimated_cost (REAL)
```

#### token_usage
```sql
- id (TEXT PRIMARY KEY)
- session_id (TEXT) -> sessions.id
- tool (TEXT) -> detected_tools.tool
- prompt_tokens (INTEGER)
- completion_tokens (INTEGER)
- total_tokens (INTEGER)
- timestamp (TIMESTAMP)
```

#### project_analysis
```sql
- id (TEXT PRIMARY KEY)
- project_path (TEXT UNIQUE)
- ai_readiness_score (INTEGER)
- total_tokens (INTEGER)
- session_count (INTEGER)
- last_analyzed (TIMESTAMP)
```

## Key Algorithms

### Tool Detection

For each supported tool, check platform-specific locations:

**Windows:**
- `%APPDATA%\<tool-name>`
- `%USERPROFILE%\.config\<tool-name>`

**macOS/Linux:**
- `~/.config/<tool-name>`
- `~/.local/share/<tool-name>`
- `~/.cache/<tool-name>`

### Token Estimation

Uses statistical approach (when tokenizer unavailable):
- ~4 characters = 1 token
- Word-based estimation: `words + words/3`

### Cost Calculation

Per provider:

**Anthropic (Claude):**
- Claude 3.5 Sonnet: $0.003/M input, $0.015/M output
- Claude 3 Opus: $0.015/M input, $0.075/M output
- Claude 3 Haiku: $0.00025/M input, $0.00125/M output

**OpenAI (GPT):**
- GPT-4: $0.03/1K input, $0.06/1K output
- GPT-4 Turbo: $0.01/1K input, $0.03/1K output
- GPT-3.5 Turbo: $0.0005/1K input, $0.0015/1K output

### AI Readiness Scoring

Score: 100 - penalties

**High priority (-10):**
- Large unexcluded directories (node_modules, venv, etc.)
- Oversized configuration files

**Medium priority (-5):**
- Duplicate instructions
- Outdated documentation

**Low priority (-2):**
- Minor configuration issues

## Extension Points

### Adding a New AI Tool

1. Update `AiTool` enum in `models.rs`
2. Add detection logic in `detection.rs`
3. Create adapter in `adapters/`
4. Update CLI help text

### Adding a New Command

1. Add variant to `Commands` enum
2. Implement handler function
3. Add to CLI parser
4. Update documentation

### Adding a New Data Source

1. Extend database schema
2. Create migration in `database.rs`
3. Add queries to `Database` impl
4. Update models

## Performance Considerations

- **Database** - Indexed queries for common operations
- **Detection** - Concurrent tool scanning
- **Memory** - Streaming for large datasets
- **Disk** - SQLite for efficient storage

## Testing Strategy

- **Unit Tests** - Per-module functionality
- **Integration Tests** - End-to-end workflows
- **Platform Tests** - Windows, macOS, Linux specific code

```bash
cargo test --workspace
cargo test --workspace -- --nocapture
```

## Security Model

- All data stored locally in `~/.ai-assist/`
- No network calls by default
- No authentication required
- No cloud sync capability
- User controls what gets tracked

## Future Architecture Changes

1. **Async Streaming** - For handling large datasets
2. **Plugin System** - Third-party adapters
3. **Event-Driven** - Real-time event processing
4. **Distributed** - Optional team analytics (future)
5. **AI Models** - Fine-tune recommendations

## Dependencies

### Core
- `tokio` - Async runtime
- `sqlx` - Type-safe SQL queries
- `serde` - Serialization
- `chrono` - Date/time handling

### CLI
- `clap` - Argument parsing
- `colored` - Terminal colors
- `indicatif` - Progress bars
- `tabled` - Table formatting

### Build
- `cargo` - Package management
- `rust 1.75+` - Compiler

## Development Workflow

1. Make changes in appropriate crate
2. Run tests: `cargo test --workspace`
3. Check formatting: `cargo fmt`
4. Lint: `cargo clippy`
5. Build release: `cargo build --release`

## Documentation

- **Code comments** - Complex algorithms only
- **Docstrings** - Public API documentation
- **README** - Quick start guide
- **ARCHITECTURE** - This file
- **CONTRIBUTING** - Development guidelines

## Known Limitations

1. Token counting is statistical estimate
2. Some tools' APIs not publicly available
3. Requires local file access
4. Desktop GUI not yet implemented
5. Real-time monitoring depends on tool logging
