#!/bin/bash

# Update local repository
sudo dnf update -y

# Install Developer tools
sudo dnf groupinstall -y "Development Tools"