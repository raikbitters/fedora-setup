# Install developer tools

import os
import subprocess

# Update local repository
os.system('sudo dnf update -y')

# Virtual Machine Manager
os.system("sudo dnf install -y @virtualization")
os.system("sudo usermod -a -G libvirt $(whoami)")