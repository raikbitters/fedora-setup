#!/bin/bash

user=$USER
bold=$(tput bold)
normal=$(tput sgr0)

function print_option_exception {
    echo -e "\n${bold}Wrong option!${normal}\n"
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
    
        [1] Install 1Password
        [2] Install Zsh with Oh My Zsh
        [3] Install Visual Studio Code
        [4] Install Docker
        [5] Install Rust
        [6] Install Go
        [7] Install Node Version Manager
        [8] Install additional fonts
        [9] Install Discord

        Configuration options:
        [10] Enable fractional scaling
        [11] Set up Git user name and email

        [0] Exit
        "
        read -p "Select option: " option

        case $option in
            0)
                exit;;
            1)
                source ./scripts/1password.sh;;
            2)
                source ./scripts/zsh.sh;;
            3) 
                source ./scripts/code.sh;;
            4)
                source ./scripts/docker.sh;;
            5) 
                source ./scripts/rust.sh;;
            6)
                source ./scripts/go.sh;;
            7)
                source ./scripts/nvm.sh;;
            8)
                source ./scripts/fonts.sh;;
            9)
                source ./scripts/discord.sh;;
            10)
                source ./scripts/scaling.sh;;
            11)
                source ./scripts/git.sh;;
            *)  
                print_option_exception;;
        esac
    done
}

# Initialization
open_main_menu
