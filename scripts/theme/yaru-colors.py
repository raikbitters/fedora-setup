# Install Yaru remix theme

import os

themesPath = "$HOME/.themes"
iconsPath = "$HOME/.icons"
assetsPath = "$HOME/Downloads/Yaru-Colors-master"

# Upload theme assets

os.system("wget -O tmp.zip https://github.com/Jannomag/Yaru-Colors/archive/master.zip && unzip -q tmp.zip -d $HOME/Downloads && rm tmp.zip")

# Make user theme directory
os.system("mkdir %s" % themesPath)
os.system("mkdir %s" % iconsPath)

# Copy assets to user theme directory
# Themes

os.system("cp -R $HOME/Downloads/Yaru-Colors-master/Themes/Yaru-Blue %s" % themesPath)
os.system("cp -R $HOME/Downloads/Yaru-Colors-master/Themes/Yaru-Blue-dark %s" % themesPath)

# Icons

os.system("cp -R $HOME/Downloads/Yaru-Colors-master/Icons/Yaru-Blue %s" % iconsPath)
os.system("cp -R $HOME/Downloads/Yaru-Colors-master/icons/Yaru-Brown %s" % iconsPath)

# Clear upload assets

os.system("rm -R %s" % assetsPath)

# Enable theme
os.system('gsettings set org.gnome.desktop.interface gtk-theme "Yaru-Blue"')
os.system('gsettings set org.gnome.desktop.interface icon-theme "Yaru-Blue"')