# Install developer tools

import os
import subprocess

# Zsh
os.system("sudo dnf install -y zsh")

# Oh My Zsh
os.system('sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended')
os.system("sudo dnf install -y util-linux-user")
os.system("chsh -s $(which zsh)")

# Fonts for Oh My Zsh
os.system("sudo dnf install -y powerline-fonts fontawesome-fonts")

# Zsh plugins
