#!/bin/bash

# Install Rust using rustup
echo -e "\nInstalling Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust to the PATH
echo -e "\nAdding Rust to PATH..."
export PATH="$HOME/.cargo/bin:$PATH"

# Optionally, you can add the PATH export to your shell profile to make it persistent
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc