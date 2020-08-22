# Fedora basic setup
My basic setup for Fedora

## Enable RPM Fusion Free
* sudo dnf install https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm
* sudo dnf update

## Install Multimedia codec package
sudo dnf groupupdate Multimedia

## Install Gnome tweaks
sudo dnf install gnome-tweaks

## Add Flathub to Gnome software
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

## Install apps from Flathub (Telegram, Bitwarden, VLC, Remmina)
flatpak install flathub org.telegram.desktop com.bitwarden.desktop org.videolan.VLC org.remmina.Remmina

## Install Chrome
sudo dnf install chrome

## Install games launchers
sudo dnf install steam
sudo dnf install lutris

## Install developmet tools

### Install fonts
* sudo dnf install powerline-fonts
* sudo dnf install fira-code-fonts
* sudo dnf install fontawesome-fonts
* sudo dnf install google-roboto-condensed-fonts
* sudo dnf install google-roboto-fonts
* sudo dnf install google-roboto-mono-fonts
* sudo dnf install google-roboto-slab-fonts

### Install zsh
* sudo dnf install zsh
* chsh -s $(which zsh)
* sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
* git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k

### Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

### Install VS Code
* sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc
* sudo sh -c 'echo -e "[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc" > /etc/yum.repos.d/vscode.repo'
* sudo dnf update
* sudo dnf install code

### Install IntelliJ IDEA and Gitg from Flathub
flatpak install flathub com.jetbrains.IntelliJ-IDEA-Community org.gnome.gitg

### Install yarn & nodejs
* curl --silent --location https://dl.yarnpkg.com/rpm/yarn.repo | sudo tee /etc/yum.repos.d/yarn.repo
* sudo dnf install yarn
