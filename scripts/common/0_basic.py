# Basic setup OS

import os

# Enable third party repositories
os.system("sudo dnf install -y fedora-workstation-repositories")
os.system("sudo dnf update -y")

# Enable RPM Fusion Free & NonFree
os.system("sudo dnf install -y https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm")
os.system("sudo dnf update -y")

# Install Multimedia codec package
os.system("sudo dnf groupupdate -y Multimedia")

# Install Gnome tweaks
os.system("sudo dnf install -y gnome-tweaks")

# Add Flathub to Gnome software
os.system("flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo")

# Install extensions
os.system("sudo dnf install -y gnome-shell-extension-dash-to-dock gnome-shell-extension-user-theme gnome-shell-extension-topicons-plus gnome-shell-extension-gsconnect")
