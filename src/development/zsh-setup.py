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
os.system("sudo dnf install -y fontawesome-fonts")
os.system("pip install --user powerline-status")

# Zsh plugins
os.system("git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting")
os.system("git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions")
os.system("git clone https://github.com/supercrabtree/k ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/k")

# Update zsh.rc 
os.system('echo -e "# Path to your oh-my-zsh installation.\nexport ZSH="${HOME}/.oh-my-zsh"\n\n# Set name of the theme.\nZSH_THEME="agnoster"\n\n# Which plugins would you like to load?\nplugins=(git sudo zsh-syntax-highlighting zsh-autosuggestions k)\n\nsource ${ZSH:-~/.oh-my-zsh}/oh-my-zsh.sh\n\n# User configuration." > ${HOME}/.zshrc')