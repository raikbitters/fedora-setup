#!/bin/bash

# Zsh
echo -e "\nZsh install"
sudo dnf install -y zsh

# Oh My Zsh
echo -e "\nOhMyZsh install"
sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended
sudo dnf install -y util-linux-user
chsh -s $(which zsh)

# OhMyZsh plugins
echo -e "\nInstall OhMyZsh plugins: zsh-syntax-highlighting zsh-autosuggestions k"
git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting
git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions
git clone https://github.com/supercrabtree/k ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/k

# OhMyZsh theme
cp -f ./assets/zsh/neo-robbyrussell.zsh-theme ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/themes/

# Copy zsh configuration
cp -f ./assets/zsh/.zshrc $HOME/.zshrc
