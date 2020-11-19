# Install developer tools

import os
import subprocess

# Update local repository
os.system('sudo dnf update -y')

# Development Tools
os.system('sudo dnf groupinstall -y "Development Tools"')

# Set JAVA_HOME
os.system('export JAVA_HOME=$(readlink -f /usr/bin/java | sed "s:bin/java::")')
os.system('export PATH=$JAVA_HOME/bin:$PATH')

# Rust
os.system("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
os.system('export PATH="$HOME/.cargo/bin:$PATH"')

# Yarn & nodejs
os.system("curl --silent --location https://dl.yarnpkg.com/rpm/yarn.repo | sudo tee /etc/yum.repos.d/yarn.repo")
os.system("sudo dnf update -y")
os.system("sudo dnf install -y yarn")

# Virtual Machine Manager
os.system("sudo dnf install -y @virtualization")
os.system("sudo usermod -a -G libvirt $(whoami)")

# Graph Visualization Software
os.system("sudo dnf install -y graphviz")