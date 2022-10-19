# Install developer fonts

import os
import subprocess

# Fonts roboto
os.system('echo -e "\nFonts roboto"')
os.system("sudo dnf install google-roboto-condensed-fonts google-roboto-fonts google-roboto-mono-fonts google-roboto-slab-fonts")

# Font for VS Code
os.system('echo -e "\nFont for VS Code"')
os.system("sudo dnf install fira-code-fonts")

# Fonts for Oh My Zsh
os.system('echo -e "\nFonts for Oh My Zsh"')
os.system("sudo dnf install fontawesome-fonts")
os.system("pip install --user powerline-status")