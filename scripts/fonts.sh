#!/bin/bash

# Install developer fonts

# Font for VS Code
echo -e "\nFont for VS Code"
sudo dnf install -y fira-code-fonts

# Fonts for Oh My Zsh
echo -e "\nFonts for Oh My Zsh"
sudo dnf install -y fontawesome-fonts-all
pip install --user powerline-status