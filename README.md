# Fedora Setup CLI

A Rust-based CLI tool for quick Fedora system setup and configuration.

## Features

This tool provides both an interactive menu and direct CLI commands for installing and configuring various software on Fedora Linux.

Key Features:

- **Pure Rust implementation**: All installation logic written in Rust using `cmd_lib`
- **Self-contained binary**: No external script files needed at runtime
- **Dual-mode operation**: Interactive menu or direct CLI commands
- **Minimal dependencies**: No heavy dependencies like `reqwest` or `openssl`
- **Interactive menu**: Easy-to-use navigation with `dialoguer`
- **CLI-friendly**: Execute installations directly via command-line arguments
- **Secure downloads**: Uses system `curl` with proper TLS settings

## Installation

### Download Prebuilt Binary (Recommended)

```bash
# Download the binary
curl -LO https://github.com/raikbitters/fedora-setup/releases/download/1.1.0/fedora-setup

# Make it executable
chmod +x fedora-setup

# Move to your PATH
sudo mv fedora-setup /usr/local/bin/
```

### Install from Source

```bash
git clone https://github.com/raikbitters/fedora-setup.git
cd fedora-setup
cargo build --release
cp target/release/fedora-setup ~/.local/bin/
```

Or install globally via cargo:

```bash
cargo install --path .
```

## Usage

The tool supports both interactive mode and direct CLI commands.

### Interactive Mode

Run without arguments to launch the interactive menu:

```bash
fedora-setup
```

Then:

1. Use arrow keys to navigate the menu
2. Press Enter to select an option
3. Follow any prompts during installation
4. Select "Exit" or "Ctrl+C" when done

### CLI Mode

Execute specific commands directly:

```bash
# Install Discord
fedora-setup discord

# Install Docker
fedora-setup docker

# Install and configure Zsh with Oh My Zsh
fedora-setup zsh

# Install Mise version manager
fedora-setup mise

# Install Zed editor
fedora-setup zed

# Set up Git user name and email
fedora-setup git
```

### Available Commands

List all available commands:

```bash
fedora-setup --list
```

### Version

Display version information:

```bash
fedora-setup --version
```

### Help

Display help information:

```bash
fedora-setup --help
```

## Development

### Prerequisites

- Fedora Linux
- Rust toolchain (install via `rustup`)
- GCC compiler (`sudo dnf install gcc` or `sudo dnf group install c-development`)

### Building

```bash
cargo build --release
```

The binary will be located at `target/release/fedora-setup`.

### Running

During development:

```bash
cargo run
```

Or run the built binary directly:

```bash
./target/release/fedora-setup
```

### Project Structure

```text
fedora-setup/
├── Cargo.toml           # Rust project dependencies
└── src/
   ├── main.rs          # Main application entry point with interactive menu
   ├── cli.rs           # CLI command definitions
   ├── menu.rs          # Interactive menu item definitions
   └── scripts/         # Installation modules (pure Rust implementations)
       ├── mod.rs
       ├── onepassword.rs
       ├── vscode.rs
       ├── zed.rs
       ├── docker.rs
       ├── zsh.rs
       ├── mise.rs
       ├── fonts.rs
       ├── discord.rs
       └── git.rs
```

### Dependencies

- `dialoguer` - Interactive CLI prompts
- `console` - Terminal utilities
- `colored` - Colored terminal output
- `anyhow` - Error handling
- `dirs` - Home directory utilities
- `cmd_lib` - Ergonomic shell command execution
- `clap` - Command-line argument parsing

### Implementation Details

All installations are implemented as pure Rust functions that use `cmd_lib::run_cmd!` macro for executing system commands. This approach provides:

- Type-safe command execution
- Proper error handling with `Result<()>`
- Clean and readable syntax similar to shell scripts
- No need for temporary script files

Example:

```rust
pub fn install_zed() -> Result<()> {
    run_cmd!(bash -c "curl -f https://zed.dev/install.sh | sh")?;
    Ok(())
}
```
