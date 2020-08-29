#!/bin/bash

user=$USER
option=""

basicPath="./src/common/basic.py"
appsPath="./src/common/apps.py"
gamesPath="./src/common/games.py"
themePath="./src/common/theme.py"
developPath="./src/development/develop-tools.py"
vscodePluginsPath="./src/development/vscode-plugins.py"
bold=$(tput bold)
normal=$(tput sgr0)

commonInstall() {
    echo "${bold}Common applications install...${normal}"
    python $basicPath
    python $appsPath
}
themeInstall() {
    echo "${bold}Theme install...${normal}"
    python $basicPath
    python $themePath
}
gamesInstall() {
    echo "${bold}Games applications install...${normal}"
    python $gamesPath
}
developInstall() {
    echo "${bold}Developper tools install...${normal}"
    python $developPath
    python $vscodePluginsPath
}
echoDone() {
    echo -e "\n${bold}Done!${normal}"
}

echo "Hello $user! It's basic Fedora setup."

while :
do
echo "
    Select install option:
    [1] Common applications.
    [2] Common and games applications.
    [3] Common and develop applications.
    [4] Common, game and develop applications.
    [5] Yaru theme install.
    [0] Exit.
"
read -p "Select option: " option

case $option in
    0)
        break
        ;;
    1)
        commonInstall
        echoDone
        ;;
    2)
        commonInstall
        gamesInstall
        echoDone
        ;;
    3)
        commonInstall
        developInstall
        echoDone
        ;;
    4)  
        commonInstall
        gamesInstall
        developInstall
        echoDone
        ;;
    5)
        themeInstall
        echoDone
        ;;
    *)
        echo "Wrong option."
        ;;
esac
done