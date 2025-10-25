# Fedora Setup CLI

A Rust-based CLI tool for quick Fedora system setup and configuration.

## Features

This tool provides an interactive menu for installing and configuring various software on Fedora Linux.

### Installation list

- [1Password](https://1password.com/)
- [Zsh](https://ohmyz.sh/)
- [Oh My Zsh with plugins](https://ohmyz.sh/)
- [Visual Studio Code](https://code.visualstudio.com/)
- [Rust](https://www.rust-lang.org/)
- [Go](https://golang.org/)
- [Node Version Manager](https://github.com/nvm-sh/nvm)
- [Docker](https://docs.docker.com/engine/install/fedora/)
- [Discord](https://discord.com/)
- Additional fonts:
  - [Fira Code](https://fonts.google.com/specimen/Fira+Code)
  - [Font Awesome](https://fontawesome.com/)
  - [Powerline status fonts](https://github.com/powerline/powerline)
  - [Roboto](https://fonts.google.com/specimen/Roboto)

### Configuration options

- Enable fractional scaling
- Set Git name and email

## Prerequisites

- Fedora Linux
- Rust toolchain (install via `rustup`)
- GCC compiler (`sudo dnf install gcc` or `sudo dnf group install c-development`)

## Building

```bash
cargo build --release
```

The binary will be located at `target/release/fedora-setup`.

## Running

From source:

```bash
cargo run
```

From binary:

```bash
./target/release/fedora-setup
```

Or install globally:

```bash
cargo install --path .
fedora-setup
```

## Usage

1. Run the program
2. Use arrow keys to navigate the menu
3. Press Enter to select an option
4. Follow any prompts during installation
5. Select "Exit" when done

## Project Structure

```text
fedora-setup/
├── Cargo.toml           # Rust project dependencies
├── src/
│   ├── main.rs          # Main application entry point with interactive menu
│   ├── menu.rs          # Menu item definitions
│   ├── installer.rs     # Installation coordinator
│   └── scripts/         # Installation modules (pure Rust implementations)
│       ├── mod.rs
│       ├── utils.rs     # Common utilities
│       ├── onepassword.rs
│       ├── vscode.rs
│       ├── docker.rs
│       ├── zsh.rs
│       ├── rust_lang.rs
│       ├── golang.rs
│       ├── nvm.rs
│       ├── fonts.rs
│       ├── discord.rs
│       ├── scaling.rs
│       └── git.rs
└── scripts/             # Original bash scripts (reference only, not used)
```

## Key Features

- **Pure Rust implementation**: All installation logic written in Rust using `cmd_lib`
- **Self-contained binary**: No external script files needed at runtime
- **Minimal dependencies**: No heavy dependencies like `reqwest` or `openssl`
- **Interactive menu**: Easy-to-use navigation with `dialoguer`
- **Secure downloads**: Uses system `curl` with proper TLS settings

## Dependencies

- `dialoguer` - Interactive CLI prompts
- `console` - Terminal utilities
- `colored` - Colored terminal output
- `anyhow` - Error handling
- `dirs` - Home directory utilities
- `cmd_lib` - Ergonomic shell command execution

## Implementation Details

All installations are implemented as pure Rust functions that use `cmd_lib::run_cmd!` macro for executing system commands. This approach provides:

- Type-safe command execution
- Proper error handling with `Result<()>`
- Clean and readable syntax similar to shell scripts
- No need for temporary script files

Example:

```rust
pub fn install_rust() -> Result<()> {
    let url = "https://sh.rustup.rs";
    run_cmd!(curl --proto "=https" --tlsv1.2 -sSf $url | sh -s -- -y)?;
    Ok(())
}
```
