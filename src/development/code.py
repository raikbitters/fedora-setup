# Microsoft Visual Studio Code setup

import os
import subprocess

os.system('echo -e "\nStart VS Code install..."')
os.system("sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc")
os.system('echo -e "[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc" | sudo tee /etc/yum.repos.d/vscode.repo')
os.system("sudo dnf update -y")
os.system("sudo dnf install -y code")