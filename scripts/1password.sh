#!/bin/bash

# Import the 1Password GPG key
sudo rpm --import https://downloads.1password.com/linux/keys/1password.asc

# Add the 1Password repository
sudo sh -c 'echo -e "[1password]\nname=1Password Stable Channel\nbaseurl=https://downloads.1password.com/linux/rpm/stable/\$basearch\nenabled=1\ngpgcheck=1\nrepo_gpgcheck=1\ngpgkey=\"https://downloads.1password.com/linux/keys/1password.asc\"" > /etc/yum.repos.d/1password.repo'

# Update the package list
sudo dnf update -y

# Install 1Password
sudo dnf install -y 1password