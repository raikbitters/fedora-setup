#!/bin/bash

user=$USER
bold=$(tput bold)
normal=$(tput sgr0)

# Common functions
function reboot () { 
    echo -e '\nReboot the system? (y/n)' && read x && [[ "$x" == "y" ]] && /sbin/reboot; 
}
function print_done {
    echo -e "\n${bold}Done!${normal}"
}

# General install functions
function install_zsh {
    echo -e "\n${bold}OhMyZsh install...${normal}\n"
    python src/zsh.py
}
function install_fonts {
    echo -e "\n${bold}Installing development fonts...${normal}\n"
    python src/fonts.py
}
function install_code {
    echo -e "\n${bold}Installing Visual Studio Code...${normal}\n"
    python src/code.py
}
function install_1password {
    echo -e "\n${bold}Installing 1Password...${normal}\n"
    python src/1password.py
}
function install_asusctl {
    echo -e "\n${bold}Installing a control daemon, CLI tools, and a collection of crates for interacting with ASUS ROG laptops...${normal}\n"
    python src/asusctl.py
}

# Programming languages install functions
function install_rust {
    echo -e "\n${bold}Installing Rust...${normal}\n"
    python ./src/development/rust.py
}
function install_nvm {
    echo -e "\n${bold}Installing Node.js...${normal}\n"
    python ./src/development/nvm.py
}
function install_dev_tools {
    echo -e "\n${bold}Installing developer tools...${normal}\n"
    python ./src/development/dev-tools.py
}

# CLI
function main_menu {
    option=""
    until [ "$option" = "0" ]; do

    echo -e "\nHello $user! It's fast Fedora setup."

    echo -e "\n${bold}Main menu${normal}"
    echo -e "
    Installation options:
   
    [1] Setup FlatPak central repository
    [2] Install OhMyZsh
    [3] Install Development fonts
    [4] Install Visual Studio Code
    [5] Install 1Password
    [6] Install Asus Ctl
    
    [9] Developer menu
    [0] Exit
    "
    read -p "Select option: " option

    case $option in
        0)
            exit;;
        1)
            flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
            print_done;;
        2)
            install_zsh
            print_done
            reboot;;
        3) 
            install_fonts
            print_done;;
        4)
            install_code
            print_done;;
        5)
            install_1password
            print_done;;
        6)
            install_asusctl
            print_done
            reboot;;
        9)
            clear
            developer_menu;;
        *)  
            echo -e "\n${bold}Wrong option${normal}";;
    esac
    done
}

function developer_menu {
    option=""
    until [ "$option" = "0" ]; do

    echo -e "\nHello $user! It's fast Fedora setup."

    echo -e "\n${bold}Developer menu${normal}"
    echo "
    Installation options:

    [1] Install Developer tools
    [2] Install Rust
    [3] Install Node version manager
    
    [9] Return to Main menu
    [0] Exit
    "
    read -p "Select option: " option

    case $option in
        0)
            exit;;
        1)
            install_dev_tools
            print_done
            reboot;;
        2)
            install_rust
            print_done;;
        3)
            install_nvm
            print_done;;
        9)
            clear
            main_menu;;
        *)  
            echo -e "\n${bold}Wrong option${normal}";;
    esac
    done
}

# Initialization

main_menu