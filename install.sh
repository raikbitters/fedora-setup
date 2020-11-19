#!/bin/bash

user=$USER
option=""

basicPath="./src/common/basic.py"
appsPath="./src/common/apps.py"
gamesPath="./src/common/games.py"
bold=$(tput bold)
normal=$(tput sgr0)

commonInstall() {
    echo "${bold}Common applications install...${normal}"
    python $basicPath
    python $appsPath
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
