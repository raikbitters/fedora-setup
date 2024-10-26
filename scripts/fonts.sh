#!/bin/bash

# Install developer fonts

# Fonts roboto
echo -e "\nFonts roboto"
sudo dnf install -y google-roboto-condensed-fonts google-roboto-fonts google-roboto-mono-fonts google-roboto-slab-fonts

# Font for VS Code
echo -e "\nFont for VS Code"
sudo dnf install -y fira-code-fonts

# Fonts for Oh My Zsh
echo -e "\nFonts for Oh My Zsh"
sudo dnf install -y fontawesome-fonts-all
pip install --user powerline-status