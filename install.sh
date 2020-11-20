#!/bin/bash

user=$USER
option=""

bold=$(tput bold)
normal=$(tput sgr0)

commonInstall() {
    echo "${bold}Common applications install...${normal}"
    python ./src/common/basic.py
    python ./src/common/apps.py
}
gamesInstall() {
    echo "${bold}Games applications install...${normal}"
    python ./src/common/games.py
}
developInstall() {
    echo "${bold}Development Tools install...${normal}"
    python ./src/development/develop-tools.py
}
zshInstall() {
    echo "${bold}Zsh package install...${normal}"
    python src/development/zsh-setup.py
}

idesInstall() {
    echo "${bold}VS Code and Intellij IDEA install...${normal}"
    python src/development/ides.py
}
printDone() {
    echo -e "\n${bold}Done!${normal}"
}

echo -e "\nHello $user! It's basic Fedora setup."

while :
do
echo "
    Pleas select install option:
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
        printDone
        ;;
    2)
        commonInstall
        gamesInstall
        printDone
        ;;
    3)
        commonInstall
        developInstall
        printDone
        ;;
    4)  
        commonInstall
        gamesInstall
        developInstall
        printDone
        ;;
    5)
        themeInstall
        printDone
        ;;
    *)
        echo "Wrong option."
        ;;
esac
done
