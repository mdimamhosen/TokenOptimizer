# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Core Engine**
  - Tool detection system for 8+ AI coding assistants
  - SQLite-based local database
  - Token counting and cost estimation
  - Optimization analysis engine
  - AI readiness scoring

- **CLI Application**
  - `ai-assist init` - Initialize and detect tools
  - `ai-assist scan` - Scan system for tools
  - `ai-assist usage` - View token usage statistics
  - `ai-assist analyze` - Project analysis and recommendations
  - `ai-assist watch` - Real-time monitoring (planned)
  - `ai-assist report` - Generate detailed reports (planned)
  - `ai-assist dashboard` - Open graphical dashboard (planned)

- **Supported Tools**
  - Claude Code
  - Cursor
  - GitHub Copilot
  - Windsurf
  - Cline
  - Continue
  - Aider
  - OpenAI Codex

- **Features**
  - Automatic tool detection
  - Cross-platform support (Windows, macOS, Linux)
  - Local-only data storage
  - Privacy-first design
  - Cost estimation for major AI providers
  - Configuration file analysis

- **Documentation**
  - README with quick start guide
  - ARCHITECTURE documentation
  - CONTRIBUTING guidelines
  - API documentation in code

### Changed

- Initial release

### Deprecated

- None

### Removed

- None

### Fixed

- None

### Security

- All data stored locally
- No telemetry by default
- No source code upload
- No prompt/conversation tracking

## [0.1.0] - 2024-08-04

### Added

- Project initialization and setup
- Core library with detection and analytics
- CLI with basic commands
- Database schema and migrations
- Tool detection for 8 AI coding assistants
- Token counting utilities
- Optimization analysis engine
- Cross-platform path handling

---

## Contributing

For unreleased changes and upcoming features, see the [GitHub Issues](https://github.com/ai-assist-dev/ai-assist/issues) and [Project Board](https://github.com/ai-assist-dev/ai-assist/projects).

## Compatibility

- **Minimum Rust Version**: 1.75
- **Minimum Node.js**: 16 (for npm package, when available)

### Platform Support

| Platform | Status | Version |
|----------|--------|---------|
| Windows 10+ | ✅ | 0.1.0+ |
| macOS 11+ | ✅ | 0.1.0+ |
| Linux (glibc) | ✅ | 0.1.0+ |

## Timeline

- **v0.1.0** (Current) - Core engine and CLI foundation
- **v0.2.0** (Planned) - Desktop dashboard with Tauri
- **v0.3.0** (Planned) - Real-time monitoring and streaming
- **v0.4.0** (Planned) - VS Code and browser extensions
- **v1.0.0** (Planned) - Production-ready release

## Installation Methods (Planned)

- **Cargo**: `cargo install ai-assist-cli`
- **Homebrew**: `brew install ai-assist`
- **Windows Installer**: Download `.exe`
- **Linux**: `apt install ai-assist` or download `.AppImage`
- **npm**: `npm install -g @ai-assist/cli`

## Known Issues

- Token counting uses statistical estimation
- Some tool APIs not publicly documented
- Desktop GUI not yet implemented
- Real-time monitoring depends on tool logging
- Team analytics not yet available

## Next Steps

See [Issues](https://github.com/ai-assist-dev/ai-assist/issues?q=is%3Aopen+is%3Aissue) for upcoming work.

---

Generated automatically. Last updated: 2024-08-04
