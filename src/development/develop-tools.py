# Install developer tools

import os
import subprocess

# Zsh
os.system("sudo dnf install -y zsh")
os.system('sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended')
os.system("sudo dnf install -y util-linux-user")
os.system("chsh -s $(which zsh)")

# Fonts
os.system("sudo dnf install -y powerline-fonts fira-code-fonts fontawesome-fonts google-roboto-condensed-fonts google-roboto-fonts google-roboto-mono-fonts google-roboto-slab-fonts")

# Set JAVA_HOME
os.system('export JAVA_HOME=$(readlink -f /usr/bin/java | sed "s:bin/java::")')
os.system('export PATH=$JAVA_HOME/bin:$PATH')

# Rust
os.system("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
os.system('export PATH="$HOME/.cargo/bin:$PATH"')

# IntelliJ IDEA and Gitg from Flathub
os.system("flatpak install -y flathub com.jetbrains.IntelliJ-IDEA-Community org.gnome.gitg")

# Yarn & nodejs
os.system("curl --silent --location https://dl.yarnpkg.com/rpm/yarn.repo | sudo tee /etc/yum.repos.d/yarn.repo")
os.system("sudo dnf update -y")
os.system("sudo dnf install -y yarn")

# VS Code
os.system("sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc")
os.system('echo -e "[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc" | sudo tee /etc/yum.repos.d/vscode.repo')
os.system("sudo dnf update -y")
os.system("sudo dnf install -y code")

#  Virtual Machine Manager\
os.system("sudo dnf install -y @virtualization")
os.system("sudo usermod -a -G libvirt $(whoami)")
