# Applications install

import os

# Flathub apps (Telegram, Bitwarden, VLC, Remmina)
os.system("flatpak install -y flathub org.telegram.desktop com.bitwarden.desktop org.videolan.VLC org.remmina.Remmina")

# Chrome
os.system("sudo dnf install -y chrome")

# Zoom
os.system("sudo dnf install -y https://zoom.us/client/latest/zoom_x86_64.rpm")