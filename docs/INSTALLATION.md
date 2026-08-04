# Installation Guide

Complete installation instructions for all platforms.

## Quick Start

### Linux / macOS

```bash
curl -sSL https://install.ai-assist.dev/install.sh | bash
```

### Windows (PowerShell)

```powershell
irm https://install.ai-assist.dev/install.ps1 | iex
```

### Cargo (All Platforms)

```bash
cargo install ai-assist-cli
```

---

## Platform-Specific Instructions

### Windows

#### Option 1: Executable Installer

1. Download `ai-assist-setup.exe` from [Releases](https://github.com/ai-assist-dev/ai-assist/releases)
2. Run the installer
3. Follow the setup wizard
4. Open Command Prompt or PowerShell
5. Run `ai-assist --version`

#### Option 2: Chocolatey

```powershell
choco install ai-assist
```

#### Option 3: Windows Package Manager

```powershell
winget install ai-assist-dev.ai-assist
```

#### Option 4: Manual Build

```powershell
# Clone repository
git clone https://github.com/ai-assist-dev/ai-assist
cd ai-assist

# Build release binary
cargo build --release

# Add to PATH
$env:Path += ";$(pwd)\target\release"

# Or install globally
cargo install --path crates/cli
```

#### Verify Installation

```powershell
ai-assist --version
ai-assist --help
```

### macOS

#### Option 1: Homebrew (Recommended)

```bash
brew tap ai-assist-dev/ai-assist
brew install ai-assist
```

#### Option 2: Binary Download

1. Download `ai-assist-macos-{intel|apple-silicon}.dmg` from [Releases](https://github.com/ai-assist-dev/ai-assist/releases)
2. Open the .dmg file
3. Drag `ai-assist` to Applications folder
4. Open Terminal and run:
   ```bash
   /Applications/ai-assist/bin/ai-assist --version
   ```

#### Option 3: Cargo

```bash
cargo install ai-assist-cli
```

#### Option 4: Manual Build

```bash
# Clone repository
git clone https://github.com/ai-assist-dev/ai-assist
cd ai-assist

# Build release binary
cargo build --release

# Install to /usr/local/bin
sudo cp target/release/ai-assist /usr/local/bin/
```

#### Verify Installation

```bash
ai-assist --version
ai-assist --help
```

### Linux

#### Option 1: AppImage

```bash
# Download
wget https://releases.ai-assist.dev/ai-assist-linux-x86_64.AppImage
chmod +x ai-assist-linux-x86_64.AppImage

# Run directly
./ai-assist-linux-x86_64.AppImage --version

# Or copy to PATH
sudo cp ai-assist-linux-x86_64.AppImage /usr/local/bin/ai-assist
ai-assist --version
```

#### Option 2: .deb Package (Debian/Ubuntu)

```bash
# Download
wget https://releases.ai-assist.dev/ai-assist_0.1.0_amd64.deb

# Install
sudo apt install ./ai-assist_0.1.0_amd64.deb

# Verify
ai-assist --version
```

#### Option 3: .rpm Package (Fedora/RHEL)

```bash
# Download
wget https://releases.ai-assist.dev/ai-assist-0.1.0-1.x86_64.rpm

# Install
sudo dnf install ./ai-assist-0.1.0-1.x86_64.rpm

# Verify
ai-assist --version
```

#### Option 4: AUR (Arch Linux)

```bash
yay -S ai-assist
# or
paru -S ai-assist
```

#### Option 5: Cargo

```bash
cargo install ai-assist-cli
```

#### Option 6: Manual Build

```bash
# Clone repository
git clone https://github.com/ai-assist-dev/ai-assist
cd ai-assist

# Build release binary
cargo build --release

# Install to /usr/local/bin
sudo cp target/release/ai-assist /usr/local/bin/
```

#### Verify Installation

```bash
ai-assist --version
ai-assist --help
```

---

## npm Package

### Installation

```bash
npm install -g @ai-assist/cli
```

### Usage

```bash
ai-assist init
ai-assist scan
ai-assist usage
```

### What it does
- Downloads correct binary for your platform
- Manages updates automatically
- Works with existing npm workflows

---

## Docker

### Official Docker Image

```bash
docker pull ai-assist-dev/ai-assist:latest
```

### Usage

```bash
# Interactive mode
docker run -it -v ~/.ai-assist:/.ai-assist ai-assist-dev/ai-assist init

# Run command
docker run -v ~/.ai-assist:/.ai-assist ai-assist-dev/ai-assist usage

# With docker-compose
docker-compose up ai-assist
```

### Docker Compose Example

```yaml
version: '3.8'
services:
  ai-assist:
    image: ai-assist-dev/ai-assist:latest
    volumes:
      - ~/.ai-assist:/.ai-assist
    command: usage
```

---

## From Source

### Prerequisites

- **Rust 1.75+** - [Install Rustup](https://rustup.rs/)
- **Git**
- **Standard build tools**

### Build Steps

```bash
# Clone repository
git clone https://github.com/ai-assist-dev/ai-assist.git
cd ai-assist

# Build release binary
cargo build --release

# Binary location
./target/release/ai-assist

# Add to PATH (Linux/macOS)
export PATH="$PATH:$(pwd)/target/release"

# Or install globally
cargo install --path crates/cli
```

### Development Build

```bash
# Faster compilation for development
cargo build

# Run without installing
cargo run -p ai-assist-cli -- --help
```

---

## Verification

After installation, verify everything works:

```bash
# Check version
ai-assist --version

# Check help
ai-assist --help

# Initialize
ai-assist init

# Scan tools
ai-assist scan

# Check usage
ai-assist usage
```

Expected output should show:
- Version number
- Help information
- Detected AI tools
- Token usage data

---

## Configuration

### Default Locations

```
~/.ai-assist/                 # Main directory
  ├── ai_assist.db            # SQLite database
  ├── config.toml             # Configuration file
  └── logs/                   # Application logs
```

### Environment Variables

```bash
# Set log level
export RUST_LOG=debug

# Enable backtraces
export RUST_BACKTRACE=1

# Custom data directory
export AI_ASSIST_DATA_DIR=/custom/path
```

### Configuration File

```toml
# ~/.ai-assist/config.toml

[general]
verbose = false
auto_scan = true
auto_save = true

[database]
path = "~/.ai-assist/ai_assist.db"

[logging]
level = "info"
format = "json"
```

---

## Update

### Using Homebrew (macOS)

```bash
brew upgrade ai-assist
```

### Using Chocolatey (Windows)

```powershell
choco upgrade ai-assist
```

### Using Cargo

```bash
cargo install --force ai-assist-cli
```

### Manual Update

1. Download latest release
2. Replace binary
3. Run `ai-assist --version`

### Check for Updates

```bash
ai-assist check-update
```

---

## Uninstall

### Homebrew (macOS)

```bash
brew uninstall ai-assist
```

### Chocolatey (Windows)

```powershell
choco uninstall ai-assist
```

### Cargo

```bash
cargo uninstall ai-assist-cli
```

### Manual Removal

```bash
# Remove binary
rm /usr/local/bin/ai-assist

# Remove data directory (optional)
rm -rf ~/.ai-assist
```

### Windows

1. Open Control Panel
2. Programs → Programs and Features
3. Find "AI Assist"
4. Click Uninstall

---

## Troubleshooting

### Installation Failed

#### On Linux/macOS
```bash
# Check Rust installation
rustc --version

# Update Rust
rustup update

# Try again
cargo install ai-assist-cli
```

#### On Windows
```powershell
# Check Rust installation
rustc --version

# Reinstall Rust
# Visit https://rustup.rs/ and reinstall

# Or use Visual Studio installer
```

### Command Not Found

```bash
# Check if installed
which ai-assist        # Linux/macOS
where ai-assist        # Windows

# Add to PATH
export PATH=$PATH:~/.cargo/bin  # Linux/macOS

# Restart terminal and try again
ai-assist --version
```

### Permission Denied

```bash
# Linux/macOS
sudo chmod +x /usr/local/bin/ai-assist

# Or install with sudo
sudo cargo install ai-assist-cli
```

### Database Error

```bash
# Remove corrupted database
rm ~/.ai-assist/ai_assist.db

# Reinitialize
ai-assist init
```

### Build Failed

```bash
# Check Rust version
rustc --version

# Update if needed
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

---

## System Requirements

| Requirement | Minimum | Recommended |
|------------|---------|-------------|
| **Rust** | 1.75 | Latest stable |
| **OS** | Windows 10 / macOS 11 / Ubuntu 20.04 | Windows 11 / macOS 12+ / Ubuntu 22.04+ |
| **CPU** | x86_64 or ARM64 | Modern multi-core |
| **RAM** | 100 MB | 256 MB |
| **Disk** | 50 MB | 100 MB |

---

## Platform Support

| Platform | Support | Format |
|----------|---------|--------|
| **Windows 10/11** | ✅ Full | .exe, Chocolatey |
| **macOS 11+** | ✅ Full | .dmg, Homebrew |
| **Linux (glibc)** | ✅ Full | .deb, .rpm, AppImage |
| **Linux (musl)** | ✅ Full | AppImage |
| **macOS ARM64** | ✅ Full | .dmg, Homebrew |

---

## Next Steps

After installation:

1. **Initialize**: `ai-assist init`
2. **Scan Tools**: `ai-assist scan`
3. **Check Usage**: `ai-assist usage`
4. **Analyze Project**: `ai-assist analyze`
5. **Read docs**: `ai-assist --help`

For more information:
- [Quick Start](../README.md#quick-start)
- [Commands Reference](COMMANDS.md)
- [Architecture](../ARCHITECTURE.md)

---

## Getting Help

- **GitHub Issues**: [Report bugs](https://github.com/ai-assist-dev/ai-assist/issues)
- **GitHub Discussions**: [Ask questions](https://github.com/ai-assist-dev/ai-assist/discussions)
- **Documentation**: [Read docs](https://docs.ai-assist.dev)

---

Last updated: 2026-08-05
