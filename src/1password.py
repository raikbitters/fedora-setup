import os
import subprocess

os.system("sudo rpm --import https://downloads.1password.com/linux/keys/1password.asc")
os.system('echo -e "[1password]\nname=1Password Stable Channel\nbaseurl=https://downloads.1password.com/linux/rpm/stable/\$basearch\nenabled=1\ngpgcheck=1\nrepo_gpgcheck=1\ngpgkey=\"https://downloads.1password.com/linux/keys/1password.asc\"" | sudo tee /etc/yum.repos.d/1password.repo')
os.system("sudo dnf update -y")
os.system("sudo dnf install 1password")