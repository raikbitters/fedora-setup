# Applications install

import os

# Flathub apps (Telegram, Bitwarden, VLC, Remmina, Spotify)
os.system("flatpak install -y flathub org.telegram.desktop com.bitwarden.desktop com.spotify.Client")

# Chrome
os.system("sudo dnf config-manager --set-enabled google-chrome")
os.system("sudo dnf update -y")
os.system("sudo dnf install -y google-chrome-stable")