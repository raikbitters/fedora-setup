#!/bin/bash

user=$USER
option=""

bold=$(tput bold)
normal=$(tput sgr0)
basic_setup() {
    echo "${bold}Basic setup...${normal}"
    python ./src/software/basic.py
}
common_install() {
    echo "${bold}Common applications install...${normal}"
    python ./src/software/apps.py
}
games_install() {
    echo "${bold}Games applications install...${normal}"
    python ./src/software/games.py
}
develop_tools_install() {
    echo "${bold}Development Tools install...${normal}"
    python ./src/development/develop-tools.py
}
zsh_install() {
    echo "${bold}Zsh package install...${normal}"
    python src/development/zsh-setup.py
}

ides_install() {
    echo "${bold}VS Code and Intellij IDEA install...${normal}"
    python src/development/ides.py
}
print_done() {
    echo -e "\n${bold}Done!${normal}"
}

echo -e "\nHello $user! It's basic Fedora setup."

#basic_setup

while :
do
echo "
If you want to install applications, select the installation option:

    [1] Common applications.
    [2] Games applications.
    [3] Develop tools.
    [4] Zsh.
    [5] VS Code and IntelliJ IDEA.
    [6] All applications.
    [7] Develop apps menu.
    [0] Exit.
    "
read -p "Select option: " option
case $option in
    *)
        echo "Wrong option."
        ;;
    0)
        break
        ;;
    1)
        common_install
        print_done
        ;;
    2)
        games_install
        print_done
        ;;
    3)  
        develop_tools_install
        print_done
        ;;
    4)
        zsh_install
        print_done
        ;;
    5)
        ides_install
        print_done
        ;;
    6)
        common_install
        games_install
        develop_tools_install
        zsh_install
        ides_install
        print_done
        ;;
    7)
        print_done
        ;;
esac
done
