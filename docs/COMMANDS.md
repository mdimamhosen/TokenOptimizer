# AI Assist Commands

Complete reference for all AI Assist CLI commands.

## Table of Contents

- [Global Options](#global-options)
- [init](#init)
- [scan](#scan)
- [usage](#usage)
- [analyze](#analyze)
- [watch](#watch)
- [report](#report)
- [dashboard](#dashboard)
- [Examples](#examples)

## Global Options

Available for all commands:

```
-v, --verbose    Enable verbose logging
-h, --help       Show help message
-V, --version    Show version information
```

## init

Initialize AI Assist and detect installed AI tools.

```bash
ai-assist init
```

### What it does:
1. Scans system for installed AI tools
2. Detects tool versions and locations
3. Creates local database
4. Initializes configuration

### Output:
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

### Use case:
First command to run when setting up AI Assist.

---

## scan

Scan and record detected AI tools in the database.

```bash
ai-assist scan
ai-assist scan --verbose
```

### What it does:
1. Scans for all supported AI tools
2. Records detection results in database
3. Updates tool configurations
4. Timestamps last detection

### Options:
None (inherits global options)

### Output:
```
Scanning for AI tools...
  ✓ Claude Code
  ✓ Continue
  ✗ Cursor
  ✗ GitHub Copilot
  ...

Scan complete!
```

### Use case:
Run periodically to update tool list and detect newly installed tools.

---

## usage

Show token usage statistics.

```bash
ai-assist usage
ai-assist usage --tool cursor
ai-assist usage --limit 100
ai-assist usage --tool claude_code --limit 50
```

### Options:
```
--tool <NAME>     Filter by specific tool
--limit <N>       Maximum records to show (default: all)
```

### Supported tool names:
- `claude_code` - Claude Code
- `cursor` - Cursor
- `github_copilot` - GitHub Copilot
- `windsurf` - Windsurf
- `cline` - Cline
- `continue` - Continue
- `aider` - Aider
- `openai_codex` - OpenAI Codex

### Output:
```
Token Usage Statistics

  1,234,567 tokens total

Current Usage by Tool:
  Claude Code  567,890 tokens
  Continue     389,456 tokens
  Cursor       277,221 tokens
```

### Use case:
Monitor how many tokens different AI tools are consuming.

---

## analyze

Analyze project for AI optimization opportunities.

```bash
ai-assist analyze
ai-assist analyze /path/to/project
ai-assist analyze . --verbose
```

### Arguments:
```
[PATH]    Project directory to analyze (default: current directory)
```

### What it does:
1. Scans project structure
2. Analyzes configuration files
3. Checks for large unexcluded directories
4. Generates AI readiness score
5. Creates optimization recommendations

### Output:
```
Analyzing project: /path/to/project

AI Readiness Score: 85/100

Found 2 Recommendations:

1. Found large directories that shouldn't be indexed: node_modules, build
   Suggestion: Exclude these directories from AI tool context: node_modules, build
   Potential savings: 40%

2. Documentation contains TODO/FIXME markers
   Suggestion: Keep documentation up-to-date. Remove or resolve TODO markers.
   Potential savings: 15%
```

### Scoring:
- 100: Perfectly optimized
- 90-99: Minor issues
- 70-89: Some improvement needed
- <70: Significant optimization opportunities

### Issues Detected:
- **Large directories**: node_modules, venv, .git, build, target, etc.
- **Configuration files**: Oversized rules files
- **Documentation**: Outdated or incomplete
- **Redundant instructions**: Duplicate configuration

### Use case:
Optimize your project setup to reduce token usage and improve AI tool efficiency.

---

## watch

Monitor real-time AI tool usage.

```bash
ai-assist watch
ai-assist watch --interval 5
ai-assist watch --interval 30 --verbose
```

### Options:
```
--interval <SECS>   Update interval in seconds (default: 2)
```

### What it displays:
- Currently active AI tool
- Active project
- Session duration
- Tokens used this session
- Estimated cost

### Output:
```
Monitoring Real-time Usage

Current Session:
  Application: Cursor
  Project: my-web-app
  Duration: 25 minutes
  Tokens: 65,432
  Cost: $1.20
```

### Use case:
Monitor current AI tool usage in real-time during work sessions.

---

## report

Generate detailed usage reports.

```bash
ai-assist report
ai-assist report --format json
ai-assist report --format csv
ai-assist report --verbose
```

### Options:
```
--format <FORMAT>   Output format (text, json, csv) (default: text)
```

### Formats:

#### text (default)
Human-readable formatted report

#### json
Machine-readable JSON format
```json
{
  "period": "last_7_days",
  "total_tokens": 1234567,
  "total_cost": 45.67,
  "by_tool": {
    "claude_code": {
      "tokens": 567890,
      "cost": 25.30
    }
  }
}
```

#### csv
Comma-separated values for spreadsheet import

### Output:
```
AI Assist Usage Report

Period: Last 7 Days
Total Tokens: 1,234,567
Total Cost: $45.67

By Tool:
  Claude Code     567,890 tokens  $25.30  46%
  Continue        389,456 tokens  $15.42  32%
  Cursor          277,221 tokens  $5.45   22%

Most Used Project:
  my-backend-api    389,456 tokens

Peak Usage Day:
  Tuesday           245,678 tokens
```

### Use case:
Generate reports for analysis, sharing with team, or cost tracking.

---

## dashboard

Open graphical dashboard.

```bash
ai-assist dashboard
```

### Features (planned):
- Interactive charts
- Real-time metrics
- Project comparisons
- Cost breakdowns
- Settings interface
- Export data

### Status: 
🔄 In Progress - Coming in v0.2.0

### Use case:
Visual overview of all AI tool usage and metrics.

---

## Examples

### Basic workflow

```bash
# First time setup
ai-assist init

# Scan current tools
ai-assist scan

# Check usage
ai-assist usage

# Analyze project
ai-assist analyze

# View detailed report
ai-assist report --format json
```

### Monitor development session

```bash
# Start watching usage
ai-assist watch --interval 5

# In another terminal, check your tokens
ai-assist usage --limit 10

# End of day report
ai-assist report
```

### Compare tools

```bash
# Check Claude Code usage
ai-assist usage --tool claude_code --limit 50

# Check Cursor usage
ai-assist usage --tool cursor --limit 50

# Compare with full report
ai-assist report --format json | grep "by_tool"
```

### Optimize multiple projects

```bash
# Analyze main project
ai-assist analyze ~/projects/main

# Analyze secondary project
ai-assist analyze ~/projects/secondary

# Compare scores
ai-assist analyze ~/projects/main | grep "Score"
ai-assist analyze ~/projects/secondary | grep "Score"
```

### Generate team report

```bash
# Export to CSV for sharing
ai-assist report --format csv > usage_report.csv

# Open in spreadsheet
open usage_report.csv

# Or JSON for processing
ai-assist report --format json > usage_report.json
```

### Debug verbose output

```bash
# See detailed logs
ai-assist --verbose scan

# See verbose usage
ai-assist --verbose usage

# With debugging
RUST_LOG=debug ai-assist analyze
```

---

## Tips

1. **Automate reporting**: Use cron jobs to generate weekly reports
2. **Monitor costs**: Check `ai-assist usage` regularly
3. **Optimize projects**: Run `ai-assist analyze` before large tasks
4. **Compare tools**: Use `--tool` flag to see which tool is most efficient
5. **Archive reports**: Save JSON reports for historical analysis

---

## Environment Variables

```bash
# Set log level
export RUST_LOG=debug
export RUST_LOG=info

# Enable backtraces
export RUST_BACKTRACE=1
export RUST_BACKTRACE=full
```

---

## Data Storage

All data is stored locally:
```
~/.ai-assist/
├── ai_assist.db          # SQLite database
├── config.toml           # Configuration
└── logs/                 # Application logs
```

---

## Troubleshooting

### Command not found
```bash
# Make sure ai-assist is installed
cargo install ai-assist-cli

# Or use full path
./target/release/ai-assist --help
```

### Database error
```bash
# Remove corrupted database
rm ~/.ai-assist/ai_assist.db

# Reinitialize
ai-assist init
```

### No tools detected
```bash
# Run with verbose output
ai-assist --verbose scan

# Check detection logic
ai-assist init
```

---

For more information, see:
- [README.md](../README.md) - Quick start
- [ARCHITECTURE.md](../ARCHITECTURE.md) - System design
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Development
