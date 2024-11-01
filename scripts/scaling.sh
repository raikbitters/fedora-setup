#!/bin/bash

# Enable fractional scaling
echo -e "\nEnabling fractional scaling..."
gsettings set org.gnome.mutter experimental-features "['scale-monitor-framebuffer']"
