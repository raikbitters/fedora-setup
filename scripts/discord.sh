#!/bin/bash

# Default Discord directory
DIR="$HOME/.local/share"
DOWNLOAD_URL="https://discord.com/api/download/stable?platform=linux&format=tar.gz"

# Download Discord
wget "$DOWNLOAD_URL" -O discord.tar.gz

# Extract Discord
rm -rf $DIR/Discord && tar -xzf discord.tar.gz -C $DIR

# Remove the downloaded archive
rm discord.tar.gz

# Create a desktop entry
cat <<EOF > $HOME/.local/share/applications/discord.desktop
[Desktop Entry]
Name=Discord
StartupWMClass=discord
Comment=All-in-one voice and text chat for gamers that's free, secure, and works on both your desktop and phone.
GenericName=Internet Messenger
Exec=$DIR/Discord/Discord
Icon=$DIR/Discord/discord.png
Type=Application
Categories=Network;InstantMessaging;
Path=/usr/bin
EOF

# Inform the user
echo -e "\nDiscord has been installed. You can launch it from the Applications menu or by running 'discord' in the terminal."