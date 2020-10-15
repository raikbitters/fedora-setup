# Commands for manual install

## Basic setup

### Enable third party repositories

`sudo dnf install fedora-workstation-repositories`

### Enable RPM Fusion Free

`sudo dnf install https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm`

`sudo dnf update`

### Enable RPM Fusion NonFree

`sudo dnf install https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm`

`sudo dnf update`

### Multimedia codec package

`sudo dnf groupupdate Multimedia`

### Gnome tweaks

`sudo dnf install gnome-tweaks`

### Add Flathub to Gnome software

`flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo`

## Common applications

### Apps from Flathub (Telegram, Bitwarden, VLC, Remmina)

`flatpak install -y flathub org.telegram.desktop com.bitwarden.desktop org.videolan.VLC org.remmina.Remmina com.spotify.Client`

### Chrome

`sudo dnf config-manager --set-enabled google-chrome`

`sudo dnf update`

`sudo dnf install chrome`

### Zoom

`sudo dnf install https://zoom.us/client/latest/zoom_x86_64.rpm`

## Games launchers

`sudo dnf install steam`

`sudo dnf install lutris`

`flatpak install flathub com.discordapp.Discord`

## Developer tools

### Fonts

`sudo dnf install powerline-fonts`

`sudo dnf install fira-code-fonts`

`sudo dnf install fontawesome-fonts`

`sudo dnf install google-roboto-condensed-fonts`

`sudo dnf install google-roboto-fonts`

`sudo dnf install google-roboto-mono-fonts`

`sudo dnf install google-roboto-slab-fonts`

### Zsh

`sudo dnf install zsh`

`sudo dnf install util-linux-user`

`chsh -s $(which zsh)`

`sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"`

### Set JAVA_HOME

`export JAVA_HOME=$(readlink -f /usr/bin/java | sed "s:bin/java::")`
`PATH=$JAVA_HOME/bin:$PATH`

### Rust

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
`export PATH="$HOME/.cargo/bin:$PATH"`

### VS Code

`sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc`

`sudo sh -c 'echo -e "[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc" > /etc/yum.repos.d/vscode.repo'`

`sudo dnf update`

`sudo dnf install code`

### IntelliJ IDEA and Gitg from Flathub

`flatpak install flathub com.jetbrains.IntelliJ-IDEA-Community org.gnome.gitg`

### Yarn & nodejs

`curl --silent --location https://dl.yarnpkg.com/rpm/yarn.repo | sudo tee /etc/yum.repos.d/yarn.repo`

`sudo dnf install yarn`

### Graph Visualization Software
`sudo dnf install graphviz`

### Virtual Machine Manager

`sudo dnf install @virtualization`

`sudo usermod -a -G libvirt $(whoami)`

If you want to enable a regular user for system administration:

```
sudo vi /etc/libvirt/libvirtd.conf
unix_sock_group = "libvirt"
unix_sock_rw_perms = "0770"
```
