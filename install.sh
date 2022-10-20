#!/bin/bash

user=$USER
bold=$(tput bold)
normal=$(tput sgr0)

# Common functions
function reboot { 
    echo -e '\nReboot the system? (y/n)' \
    && read x \
    && [[ "$x" == "y" ]] \
    && /sbin/reboot
}

function print_option_exception {
    echo -e "\n${bold}Wrong option!${normal}\n"
}

# General install functions
function setup_flathub {
    echo -e "\n${bold}Enable Flathub...${normal}"
    flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
    echo -e "Done!"
}

function install_zsh {
    echo -e "\n${bold}Zsh and Oh My Zsh install...${normal}\n"
    python src/zsh.py
    reboot
}

function install_fonts {
    echo -e "\n${bold}Installing additional fonts...${normal}\n"
    python src/fonts.py
}

function install_1password {
    echo -e "\n${bold}Installing 1Password...${normal}\n"
    python src/1password.py
}

function install_asusctl {
    echo -e "\n${bold}Installing a control daemon, CLI tools, and a collection of crates for interacting with ASUS ROG laptops...${normal}\n"
    python src/asusctl.py
    reboot
}

# Programming languages install functions
function install_code {
    echo -e "\n${bold}Installing Visual Studio Code...${normal}\n"
    python src/code.py
}

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
    reboot
}

function install_docker {
    echo -e "\n${bold}Installing Docker...${normal}\n"
    python ./src/development/docker.py
    setup_docker_non_root
    reboot
}

function setup_docker_non_root {
    echo -e "\n${bold}To create the docker group and add your user...${normal}\n"
    sudo groupadd docker && \
    sudo usermod -aG docker $USER && \
    newgrp docker
    echo -e "\nTo verify that you can run docker commands without sudo after reboot!"
}

# CLI
function open_main_menu {
    clear
    option=""
    until [ "$option" = "0" ]; do

    echo -e "\nHello $user! It's fast Fedora setup."

    echo -e "\n${bold}Main menu${normal}"
    echo -e "
    Installation options:
   
    [1] Setup FlatPak central repository
    [2] Install Zsh with Oh My Zsh
    [3] Install additional fonts
    [4] Install 1Password
    [5] Install ROG Asus utilities
    
    [9] Developer menu
    [0] Exit
    "
    read -p "Select option: " option

    case $option in
        0)
            exit;;
        1)
            setup_flathub;;
        2)
            install_zsh;;
        3) 
            install_fonts;;
        4)
            install_1password;;
        5)
            install_asusctl;;
        9)         
            open_developer_menu;;
        *)  
            print_option_exception;;

    esac
    done
}

function open_developer_menu {
    clear
    option=""
    until [ "$option" = "0" ]; do

    echo -e "\nHello $user! It's fast Fedora setup."

    echo -e "\n${bold}Developer menu${normal}"
    echo "
    Installation options:

    [1] Install Visual Studio Code
    [2] Install Docker
    [3] Install Rust
    [4] Install Node version manager
    [5] Install Developer tools
    
    [9] Return to Main menu
    [0] Exit
    "
    read -p "Select option: " option

    case $option in
        0)
            exit;;
        1)
            install_code;;
        2)
            install_docker;;
        3)
            install_rust;;
        4)
            install_nvm;;
        5)
            install_dev_tools;;
        9)
            open_main_menu;;
        *)  
            print_option_exception;;
    esac
    done
}

# Initialization
open_main_menu