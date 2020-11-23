# IDEs setup

import os
import subprocess

# VS Code
os.system('echo -e "\nStart VS Code install..."')
os.system("sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc")
os.system('echo -e "[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc" | sudo tee /etc/yum.repos.d/vscode.repo')
os.system("sudo dnf update -y")
os.system("sudo dnf install -y code")

# VS Code plugins
os.system('echo -e "\nStart VS Code plugins install..."')
os.system("code --install-extension 42Crunch.vscode-openapi")
os.system("code --install-extension bungcip.better-toml")
os.system("code --install-extension DavidAnson.vscode-markdownlint")
os.system("code --install-extension dbaeumer.vscode-eslint")
os.system("code --install-extension jebbs.plantuml")
os.system("code --install-extension ms-azuretools.vscode-docker")
os.system("code --install-extension ms-python.python")
os.system("code --install-extension ms-toolsai.jupyter")
os.system("code --install-extension msysyamamoto.vscode-fluentd")
os.system("code --install-extension redhat.vscode-yaml")
os.system("code --install-extension rust-lang.rust")
os.system("code --install-extension serayuzgur.crates")
os.system("code --install-extension stoplight.spectral")
os.system("code --install-extension streetsidesoftware.code-spell-checker")
os.system("code --install-extension streetsidesoftware.code-spell-checker-russian")
os.system("code --install-extension vadimcn.vscode-lldb")
os.system("code --install-extension yzhang.markdown-all-in-one")

# Font for VS Code
os.system("sudo dnf install -y fira-code-fonts")

# IntelliJ IDEA from Flathub
os.system("flatpak install -y flathub com.jetbrains.IntelliJ-IDEA-Community")