# Install Yaru remix theme

import os

#themesPath = "$HOME/.themes"
#iconsPath = "$HOME/.icons"
assetsThemesPath = "./assets/.themes"
assetsIconsPath = "./assets/.icons"

# Copy assets to user theme directory
os.system("cp -R %s $HOME/" % assetsThemesPath)
os.system("cp -R %s $HOME/" % assetsIconsPath)

# Enable theme
os.system('gsettings set org.gnome.desktop.interface gtk-theme "Yaru-Blue"')
os.system('gsettings set org.gnome.desktop.interface icon-theme "Yaru-remix"')