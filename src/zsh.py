import os
import subprocess

# Zsh
os.system('echo -e "\nZsh install"')
os.system("sudo dnf install zsh")

# Oh My Zsh
os.system('echo -e "\nOhMyZsh install"')
os.system('sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended')
os.system("sudo dnf install util-linux-user")
os.system("chsh -s $(which zsh)")

# OhMyZsh plugins
os.system('echo -e "\nInstall OhMyZsh plugins: zsh-syntax-highlighting zsh-autosuggestions k"')
os.system("git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting")
os.system("git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions")
os.system("git clone https://github.com/supercrabtree/k ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/k")