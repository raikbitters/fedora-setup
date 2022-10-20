import os
import subprocess

# Set up the repository
os.system('sudo dnf -y install dnf-plugins-core')
os.system('sudo dnf config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo')

# Install Docker Engine
os.system('sudo dnf install docker-ce docker-ce-cli containerd.io docker-compose-plugin')
os.system('sudo systemctl start docker')

# Enable Docker Engine
os.system('sudo systemctl enable docker.service')
os.system('sudo systemctl enable containerd.service')
