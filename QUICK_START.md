# AI Assist - Quick Start Guide

Get up and running with AI Assist in 5 minutes.

## Installation

### Option 1: From Cargo (Recommended)

```bash
cargo install ai-assist-cli
```

### Option 2: Build from Source

```bash
git clone https://github.com/ai-assist-dev/ai-assist
cd ai-assist
cargo build --release
./target/release/ai-assist --help
```

## First Steps

### 1. Initialize (Auto-Detect Tools)

```bash
ai-assist init
```

Expected output:
```
Initializing AI Assist...

8 AI Tools Detected:

  ✓ Claude Code
     Path: ~/.claude
  ✗ Cursor
  ✓ Continue
     Path: ~/.continue
  ...

Database initialized at:
  ~/.ai-assist/ai_assist.db
```

### 2. Scan System

```bash
ai-assist scan
```

This records detected tools in the local database.

### 3. View Usage

```bash
ai-assist usage
```

Shows total tokens and breakdown by tool.

### 4. Analyze Project

```bash
ai-assist analyze
```

Generates AI readiness score with improvement recommendations.

## Common Commands

### Check Specific Tool

```bash
ai-assist usage --tool cursor
ai-assist usage --tool claude_code
```

### Analyze Different Project

```bash
ai-assist analyze /path/to/project
```

### View Detailed Report

```bash
ai-assist report
ai-assist report --format json
ai-assist report --format csv
```

### Watch Real-time

```bash
ai-assist watch
ai-assist watch --interval 5
```

## What Gets Stored?

Everything is stored locally in `~/.ai-assist/`:

```
~/.ai-assist/
├── ai_assist.db    # SQLite database (your data)
└── (no cloud sync, no telemetry)
```

**Privacy**: No data leaves your computer.

## Understanding the Output

### AI Readiness Score

```
Score: 90/100

What it means:
- 90-100: Excellent (minimal optimization needed)
- 70-89:  Good (some improvements possible)
- 50-69:  Fair (several optimization opportunities)
- <50:    Poor (significant issues detected)
```

### Recommendations

```
1. Found large directories [HIGH PRIORITY]
   Current: node_modules, build, target
   Action: Exclude from AI tool context
   Savings: ~40% token reduction
```

Priority levels:
- **HIGH** (-10 points) - Major efficiency issue
- **MEDIUM** (-5 points) - Some impact
- **LOW** (-2 points) - Minor concern

## Typical Workflow

```bash
# Morning
ai-assist scan          # Update tool detection
ai-assist usage         # Check daily usage

# During development
ai-assist watch         # Monitor current session
ai-assist usage         # Quick check

# End of week
ai-assist report        # Review usage
ai-assist analyze .     # Check project setup

# Export for sharing
ai-assist report --format json > weekly_report.json
ai-assist report --format csv > weekly_report.csv
```

## Troubleshooting

### Command Not Found

```bash
# Make sure it's installed
cargo install ai-assist-cli

# Or use full path
~/.cargo/bin/ai-assist --help
```

### No Tools Detected

```bash
# Check supported tools are installed
ai-assist init

# See specific details
ai-assist --verbose init
```

### Database Error

```bash
# Reset database
rm ~/.ai-assist/ai_assist.db

# Reinitialize
ai-assist init
```

## Next Steps

- **Read Full Docs**: See [README.md](README.md)
- **Learn Commands**: See [docs/COMMANDS.md](docs/COMMANDS.md)
- **Install Options**: See [docs/INSTALLATION.md](docs/INSTALLATION.md)
- **Develop**: See [CONTRIBUTING.md](CONTRIBUTING.md)

## Key Features

✅ **Auto-Detection** - Finds installed AI tools automatically  
✅ **Universal** - Works with 8+ different AI assistants  
✅ **Private** - All data stored locally  
✅ **Cross-Platform** - Windows, macOS, Linux  
✅ **Free** - Open source MIT license  

## Supported Tools

- Claude Code ✓
- Cursor ✓
- GitHub Copilot ✓
- Windsurf ✓
- Cline ✓
- Continue ✓
- Aider ✓
- OpenAI Codex ✓

## Data Stored

When you use AI Assist, we track:
- ✅ Which tools are installed
- ✅ Total tokens used
- ✅ Cost estimates
- ✅ Project configurations

We DON'T track:
- ❌ Your code or prompts
- ❌ Private information
- ❌ Cloud data
- ❌ Any telemetry

## Questions?

- **Issues**: [GitHub Issues](https://github.com/ai-assist-dev/ai-assist/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ai-assist-dev/ai-assist/discussions)
- **Docs**: [Documentation](docs/)

---

**That's it!** You're ready to use AI Assist. Start with `ai-assist init` and explore from there.
