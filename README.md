# AI Assist

> Universal analytics platform for AI coding assistants

A production-grade cross-platform developer tool that provides analytics and optimization insights for AI coding assistants.

## Features

### Core Capabilities

- **🔍 Auto Detection** - Automatically detects installed AI tools (Claude Code, Cursor, GitHub Copilot, Windsurf, Cline, Continue, Aider, and more)
- **📊 Universal Analytics** - Unified token usage tracking across all AI tools
- **💰 Cost Analysis** - Estimated costs for token usage by tool and project
- **⚙️ Optimization Engine** - Intelligent recommendations for reducing token waste
- **🎯 Project Analysis** - AI readiness scoring for projects
- **📈 Real-time Monitoring** - Watch current AI tool usage in real-time
- **💾 Local Database** - All data stored locally with zero cloud upload

### Security & Privacy

- ✅ No cloud account required
- ✅ No telemetry by default
- ✅ No source code upload
- ✅ No prompt/conversation upload
- ✅ Everything runs locally

## Installation

### From Source

Requires Rust 1.75+

```bash
git clone https://github.com/ai-assist-dev/ai-assist
cd ai-assist
cargo build --release
cargo install --path crates/cli
```

### macOS (Homebrew)

```bash
brew tap ai-assist-dev/ai-assist
brew install ai-assist
```

### Windows

Download the `.exe` installer from releases or use:

```bash
choco install ai-assist
```

### Linux

Download `.deb` or `.AppImage` from releases.

## Quick Start

### Initialize

```bash
ai-assist init
```

Detects all installed AI tools on your system.

### Scan

```bash
ai-assist scan
```

Scans and records detected AI tools and their configurations.

### View Usage

```bash
ai-assist usage
ai-assist usage --tool cursor
ai-assist usage --limit 100
```

Shows token usage statistics.

### Analyze Project

```bash
ai-assist analyze
ai-assist analyze /path/to/project
```

Generates AI readiness score and optimization recommendations.

### Watch Real-time

```bash
ai-assist watch
ai-assist watch --interval 5
```

Monitor current AI tool usage in real-time.

### Generate Report

```bash
ai-assist report
ai-assist report --format json
```

Generate detailed usage reports.

### Dashboard

```bash
ai-assist dashboard
```

Opens the graphical dashboard (coming soon).

## Architecture

### Core (`crates/core`)

- **Detection Engine** - Identifies installed AI tools and their configurations
- **Analytics Engine** - Processes and analyzes usage data
- **Token Counter** - Estimates tokens and costs for different models
- **Optimization Engine** - Generates improvement recommendations
- **Database** - SQLite-based local data storage
- **Adapters** - Tool-specific data parsers

### CLI (`crates/cli`)

Command-line interface with colored output and interactive features.

### Desktop (`crates/desktop`)

Tauri-based desktop application (in progress).

### Adapters (`crates/adapters`)

Tool-specific adapters for:
- Claude Code
- Cursor
- GitHub Copilot
- Windsurf
- Cline
- Continue
- Aider
- OpenAI Codex

## Configuration

Data is stored in `~/.ai-assist/`

```
~/.ai-assist/
├── ai_assist.db          # Local SQLite database
├── config.toml           # Configuration file
└── logs/                 # Application logs
```

## Development

### Setup

```bash
git clone https://github.com/ai-assist-dev/ai-assist
cd ai-assist
cargo build
```

### Run Tests

```bash
cargo test --workspace
```

### Run CLI

```bash
cargo run -p ai-assist-cli -- --help
```

### Build Release

```bash
cargo build --release --workspace
```

## Platform Support

| Platform | Status | Format |
|----------|--------|--------|
| **Windows 10+** | ✅ | .exe installer |
| **macOS 11+** | ✅ | .dmg, Homebrew |
| **Linux** | ✅ | .deb, AppImage |

## Supported AI Tools

| Tool | Detection | Analytics | Config Analysis |
|------|-----------|-----------|-----------------|
| Claude Code | ✅ | 🔄 | 🔄 |
| Cursor | ✅ | 🔄 | 🔄 |
| GitHub Copilot | ✅ | 🔄 | 🔄 |
| Windsurf | ✅ | 🔄 | 🔄 |
| Cline | ✅ | 🔄 | 🔄 |
| Continue | ✅ | 🔄 | 🔄 |
| Aider | ✅ | 🔄 | 🔄 |
| OpenAI Codex | ✅ | 🔄 | 🔄 |

Legend: ✅ = Complete | 🔄 = In Progress

## Roadmap

### Phase 1 (Current)
- [x] Core architecture design
- [x] Tool detection engine
- [x] Database schema
- [x] CLI scaffolding
- [ ] Complete CLI commands
- [ ] Adapter implementations

### Phase 2 (Next)
- [ ] Desktop dashboard (Tauri)
- [ ] Real-time monitoring
- [ ] Prompt analysis
- [ ] Advanced optimization engine

### Phase 3 (Future)
- [ ] VS Code extension
- [ ] Browser extension
- [ ] Team analytics
- [ ] AI model recommendations
- [ ] Context compression

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) file

## Support

- **Issues**: Report bugs on [GitHub Issues](https://github.com/ai-assist-dev/ai-assist/issues)
- **Discussions**: Join [GitHub Discussions](https://github.com/ai-assist-dev/ai-assist/discussions)
- **Docs**: Full documentation at [docs/](docs/)

## Acknowledgments

Inspired by:
- **btop** - System monitoring
- **lazygit** - Developer workflow
- **Docker CLI** - Cross-platform tooling
- **VS Code** - Analytics and profiling

---

Built with ❤️ for developers who use AI coding assistants
# TokenOptimizer
