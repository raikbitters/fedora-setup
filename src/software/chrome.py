# Applications install

import os

# Chrome
os.system("sudo dnf config-manager --set-enabled google-chrome")
os.system("sudo dnf update -y")
os.system("sudo dnf install -y google-chrome-stable")