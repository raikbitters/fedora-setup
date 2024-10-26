#!/bin/bash

# Set up the repository
sudo dnf -y install dnf-plugins-core
sudo dnf config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo

# Install Docker Engine
sudo dnf install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
sudo systemctl start docker

# Enable Docker Engine
sudo systemctl enable docker.service
sudo systemctl enable containerd.service

# Add the current user to the docker group
sudo usermod -aG docker $USER

# Inform the user to log out and back in
echo -e "\nPlease log out and log back in so that your group membership is re-evaluated."

# Start Docker at boot
sudo systemctl enable docker.service
sudo systemctl enable containerd.service