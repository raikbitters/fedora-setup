import os
import subprocess

# Rust
os.system("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
os.system('export PATH="$HOME/.cargo/bin:$PATH"')