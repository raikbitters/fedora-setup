# Install developer fonts

import os
import subprocess

# Fonts roboto
os.system("sudo dnf install -y google-roboto-condensed-fonts google-roboto-fonts google-roboto-mono-fonts google-roboto-slab-fonts")

# Font for VS Code
os.system("sudo dnf install -y fira-code-fonts")

# Fonts for Oh My Zsh
os.system("sudo dnf install -y fontawesome-fonts")
os.system("pip install --user powerline-status")