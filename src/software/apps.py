# Applications install

import os

os.system("sudo dnf update -y")

# Install Gnome tweaks
os.system("sudo dnf install -y gnome-tweaks")

# Install extensions
os.system("sudo dnf install -y gnome-shell-extension-dash-to-dock gnome-shell-extension-user-theme gnome-shell-extension-topicons-plus gnome-shell-extension-gsconnect")

# Flathub apps (Bitwarden, Remmina, Spotify)
os.system("flatpak install -y flathub com.bitwarden.desktop com.spotify.Client org.remmina.Remmina")

# Telegram
os.system("wget https://telegram.org/dl/desktop/linux")
os.system("tar -xvf ./linux -C $HOME/.local/share/ && rm ./linux")

# Zoom
os.system("sudo dnf install -y https://zoom.us/client/latest/zoom_x86_64.rpm")

# Chrome
os.system("sudo dnf config-manager --set-enabled google-chrome")
os.system("sudo dnf update -y")
os.system("sudo dnf install -y google-chrome-stable")