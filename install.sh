#!/bin/bash

user=$USER
bold=$(tput bold)
normal=$(tput sgr0)

function basic_setup {
    echo -e "\n${bold}Basic setup...${normal}\n"
    python ./src/software/basic.py
}
function common_install {
    echo -e "\n${bold}Common applications install...${normal}\n"
    python ./src/software/apps.py
}
function games_install {
    echo -e "\n${bold}Games applications install...${normal}\n"
    python ./src/software/games.py
}
function develop_tools_install {
    echo -e "\n${bold}Development Tools install...${normal}\n"
    python ./src/development/develop-tools.py
}
function zsh_install {
    echo -e "\n${bold}Zsh package install...${normal}\n"
    python src/development/zsh-setup.py
}
function ides_install {
    echo -e "\n${bold}VS Code and Intellij IDEA install...${normal}\n"
    python src/development/ides.py
}
function print_done {
    echo -e "\n${bold}Done!${normal}"
}
function developer_menu {
    option=""
    until [ "$option" = "0" ]; do

    echo -e "\n${bold}Developer menu${normal}"
    echo "
    Installation options:

    [1] Develop tools install
    [2] Oh My Zsh install
    [3] VS Code and IntelliJ IDEA install
    [4] All developer apps install

    Other options:
    
    [0] Return to Main menu
    "
    read -p "Select option: " option

    case $option in
        0)
            break;;
        1)
            develop_tools_install
            print_done;;
        2)
            zsh_install
            print_done;;
        3)
            ides_install1
            print_done;;
        4)
            develop_tools_install
            ides_install
            zsh_install
            print_done;;
        *)  
            echo -e "\n${bold}Wrong option${normal}";;
    esac
    done
}
function main_menu {
    option=""
    until [ "$option" = "4" ]; do

    echo -e "\n${bold}Main menu${normal}"
    echo -e "
    Installation options:
    
    [1] Common applications
    [2] Games applications
    [3] All applications

    Other options:
    
    [4] Developer menu
    [0] Exit
    "
    read -p "Select option: " option

    case $option in
        0)
            exit;;
        1)
            common_install
            print_done;;
        2)
            games_install
            print_done;;
        3)
            common_install
            games_install
            print_done;;
        4)
            developer_menu;;
        *)  
            echo -e "\n${bold}Wrong option${normal}";;
    esac
    done
}

echo -e "\nHello $user! It's fast Fedora setup."

basic_setup

main_menu