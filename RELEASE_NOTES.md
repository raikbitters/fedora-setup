# Release Notes

## Version 1.0.0 - Initial Release

**Release Date:** October 25, 2025

First public release of Fedora Setup CLI - a modern tool for quick installation and configuration of software on Fedora Linux.

### Key Features

#### Dual-Mode Operation

- **Interactive Menu** - Easy navigation using arrow keys and Enter
- **CLI Commands** - Direct command execution via command-line arguments
- Support for `--list` and `--help` flags for quick access to information

#### Architecture

- Fully written in Rust for maximum performance and safety
- Self-contained binary - no external script files required at runtime
- Minimal dependencies - no heavy libraries like `reqwest` or `openssl`
- Uses system `curl` with proper TLS settings
- Clean and maintainable codebase with ergonomic command execution

### Available Installers

#### Applications

- **1Password** - Password manager
- **Discord** - Community messaging platform
- **Visual Studio Code** - Microsoft's code editor

#### Development Tools

- **Zsh** - Advanced shell with Oh My Zsh configuration
- **Docker** - Container platform
- **Rust** - Programming language via rustup
- **Go** - Programming language from Google
- **NVM** - Node Version Manager for managing Node.js versions
- **Fonts** - Additional fonts for terminal and editors

#### System Configuration

- **Git** - Configure user name and email
- **Fractional Scaling** - Enable/disable fractional scaling in GNOME

### Usage Examples

```bash
# Interactive mode
fedora-setup

# Install specific package
fedora-setup docker
fedora-setup vscode
fedora-setup zsh

# List all available commands
fedora-setup --list

# Show version
fedora-setup --version

# Show help
fedora-setup --help
```

### Installation

**Download prebuilt binary (recommended):**

```bash
# Download the binary
curl -LO https://github.com/raikbitters/fedora-setup/releases/download/1.0.0/fedora-setup

# Make it executable
chmod +x fedora-setup

# Move to your PATH
sudo mv fedora-setup /usr/local/bin/
```

**From source:**

```bash
git clone https://github.com/raikbitters/fedora-setup.git
cd fedora-setup
cargo build --release
sudo cp target/release/fedora-setup /usr/local/bin/
```

**Or install via cargo:**

```bash
cargo install --path .
```

### Known Limitations

- Works only on Fedora Linux
- Some installers require sudo privileges
- Active internet connection required for downloading packages

### What's Next

Future versions may include:

- Support for additional distributions
- More software installers
- Configuration backup and restore
- Plugin system for custom installers

### Acknowledgments

Thanks to all developers of the libraries used in this project and to the Fedora community for an excellent distribution.

---

For bug reports and feature requests, please use GitHub Issues.
