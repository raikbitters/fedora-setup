# Basic setup OS

import os

# Enable third party repositories
os.system("sudo dnf install -y fedora-workstation-repositories")

# Add Flathub to Gnome software
os.system("flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo")

# Enable RPM Fusion Free & NonFree
os.system("sudo dnf install -y https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm")

os.system("sudo dnf update -y")

# Install Multimedia codec package
os.system("sudo dnf groupupdate -y Multimedia")

# Fonts
os.system("sudo dnf install -y google-roboto-condensed-fonts google-roboto-fonts google-roboto-mono-fonts google-roboto-slab-fonts")