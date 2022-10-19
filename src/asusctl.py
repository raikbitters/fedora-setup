# Asus Ctl for ASUS ROG

import os
import subprocess

os.system("sudo dnf copr enable lukenukem/asus-linux")
os.system("sudo dnf update -y")
os.system("sudo dnf install asusctl supergfxctl")
os.system("sudo systemctl enable supergfxd.service")
os.system("sudo dnf install asusctl-rog-gui")