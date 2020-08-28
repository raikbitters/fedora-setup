# Fedora basic setup

My basic setup for Fedora

## Enable RPM Fusion Free

* sudo dnf install https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm
* sudo dnf update

## Enable RPM Fusion NonFree

* sudo dnf install https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm
* sudo dnf update

## Multimedia codec package

sudo dnf groupupdate Multimedia

## Gnome tweaks

sudo dnf install gnome-tweaks

## Add Flathub to Gnome software

flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

## Apps from Flathub (Telegram, Bitwarden, VLC, Remmina)

flatpak install flathub org.telegram.desktop com.bitwarden.desktop org.videolan.VLC org.remmina.Remmina

## Chrome

sudo dnf install chrome

## Zoom

sudo dnf install https://zoom.us/client/latest/zoom_x86_64.rpm

## Games launchers

sudo dnf install steam
sudo dnf install lutris

## Install developer tools

### Fonts

* sudo dnf install powerline-fonts
* sudo dnf install fira-code-fonts
* sudo dnf install fontawesome-fonts
* sudo dnf install google-roboto-condensed-fonts
* sudo dnf install google-roboto-fonts
* sudo dnf install google-roboto-mono-fonts
* sudo dnf install google-roboto-slab-fonts

### Zsh

* sudo dnf install zsh
* chsh -s $(which zsh)
* sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

### Rust

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

### VS Code

* sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc
* sudo sh -c 'echo -e "[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc" > /etc/yum.repos.d/vscode.repo'
* sudo dnf update
* sudo dnf install code

### IntelliJ IDEA and Gitg from Flathub

flatpak install flathub com.jetbrains.IntelliJ-IDEA-Community org.gnome.gitg

### Yarn & nodejs

* curl --silent --location https://dl.yarnpkg.com/rpm/yarn.repo | sudo tee /etc/yum.repos.d/yarn.repo
* sudo dnf install yarn

### VS Code plugins

* 42Crunch.vscode-openapi
* Arjun.swagger-viewer
* bungcip.better-toml
* DavidAnson.vscode-markdownlint
* dbaeumer.vscode-eslint
* Equinusocio.vsc-community-material-theme
* Equinusocio.vsc-material-theme
* equinusocio.vsc-material-theme-icons
* ms-azuretools.vscode-docker
* ms-python.python
* redhat.vscode-yaml
* rust-lang.rust
* serayuzgur.crates
* stoplight.spectral
* streetsidesoftware.code-spell-checker
* streetsidesoftware.code-spell-checker-russian
* vadimcn.vscode-lldb
