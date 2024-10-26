#!/bin/bash

# Microsoft Visual Studio Code setup

# Import the Microsoft GPG key
sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc

# Add the Visual Studio Code repository
echo -e "[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc" | sudo tee /etc/yum.repos.d/vscode.repo > /dev/null

# Update the package list
sudo dnf update -y

# Install Visual Studio Code
sudo dnf install -y code