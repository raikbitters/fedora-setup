#!/bin/bash

user=$USER
bold=$(tput bold)
normal=$(tput sgr0)

# Software install functions
function repositories_setup {
    echo -e "\n${bold}Basic setup...${normal}\n"
    python ./src/software/repositories.py
}
function tweaks {
    echo -e "\n${bold}Install gnome tweaks...${normal}\n"
    python ./src/software/tweaks.py
}
function telegram {
    echo -e "\n${bold}Install Telegram...${normal}\n"
    python ./src/software/telegram.py
}
function chrome {
    echo -e "\n${bold}Install Chrome...${normal}\n"
    python ./src/software/chrome.py
}
function zoom {
    echo -e "\n${bold}Install Zoom...${normal}\n"
    python ./src/software/zoom.py
}
function games_install {
    echo -e "\n${bold}Games applications install...${normal}\n"
    python ./src/software/games.py
}

# Develop install functions
function zsh_setup {
    echo -e "\n${bold}Zsh package install...${normal}\n"
    python src/development/zsh.py
}
function dev_fonts {
    echo -e "\n${bold}Installing development fonts...${normal}\n"
    python src/development/fonts.py
}
function code {
    echo -e "\n${bold}Installing Visual Studio Code...${normal}\n"
    python src/development/code.py
}
function print_done {
    echo -e "\n${bold}Done!${normal}"
}

# Programming languages install functions
function rust_install {
    echo -e "\n${bold}Installing Rust...${normal}\n"
    python ./src/development/rust.py
}
function nvm_install {
    echo -e "\n${bold}Installing Node.js...${normal}\n"
    python ./src/development/nvm.py
}
function yarn_install {
    echo -e "\n${bold}Installing Node.js...${normal}\n"
    python ./src/development/yarn.py
}
function c_plus_install {
    echo -e "\n${bold}Installing C++...${normal}\n"
    python ./src/development/c_plus.py
}
function set_java_home {
    echo -e "\n${bold}Set $JAVA_HOME...${normal}\n"
    python ./src/development/java_home.py
}
function virtualization_install {
    echo -e "\n${bold}Installing Virtualization...${normal}\n"
    python ./src/development/virtualization.py
}

# CLI
function main_menu {
    option=""
    until [ "$option" = "0" ]; do

    echo -e "\nHello $user! It's fast Fedora setup."

    echo -e "\n${bold}Main menu${normal}"
    echo -e "
    Installation options:
    
    [1] Setup third party repositories
    [2] Gnome tweaks
    [3] Telegram
    [4] Chrome
    [5] Zoom
    [6] Games applications (Steam, Lutris, Discord)
    [7] All applications
    
    Other options:

    [8] Turn on Fractional scaling
    [9] Developer menu
    [0] Exit
    "
    read -p "Select option: " option

    case $option in
        0)
            exit;;
        1)
            repositories_setup
            print_done;;
        2)
            tweaks
            print_done;;
        3)
            telegram
            print_done;;
        4)
            chrome
            print_done;;
        5)
            zoom
            print_done;;
        6)
            games_install
            print_done;;
        7)
            repositories_setup
            tweaks
            telegram
            chrome
            zoom
            games_install
            print_done;;
        8)
            gsettings set org.gnome.mutter experimental-features "['scale-monitor-framebuffer']"
            print_done;;
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

    [1] Zsh (Oh My Zsh)
    [2] Develop fonts
    [3] Visual Studio Code

    Other options:

    [4] Programming languages menu
    [0] Return to Main menu
    "
    read -p "Select option: " option

    case $option in
        0)
            clear
            main_menu;;
        1)
            zsh_setup
            print_done;;
        2)
            dev_fonts
            print_done;;
        3)
            code
            print_done;;
        4)
            clear
            programming_menu;;
        *)  
            echo -e "\n${bold}Wrong option${normal}";;
    esac
    done
}

function programming_menu {
    option=""
    until [ "$option" = "0" ]; do

    echo -e "\nHello $user! It's fast Fedora setup."

    echo -e "\n${bold}Programming languages menu${normal}"
    echo "
    Installation options:

    [1] Rust
    [2] Node Version Manager
    [3] Yarn and Node.js
    [4] C++
    [5] Set JAVA_HOME
    [6] Virtualization

    Other options:
    
    [6] Return to Developer menu
    [0] Return to Main menu
    "
    read -p "Select option: " option

    case $option in
        0)
            clear
            main_menu;;
        1)
            rust_install
            print_done;;
        2)
            nvm_install
            print_done;;
        3)
            yarn_install
            print_done;;
        3)
            c_plus_install
            print_done;;
        4)
            set_java_home
            print_done;;
        5)
            virtualization_install
            print_done;;
        6)
            clear
            developer_menu;;
        *)  
            echo -e "\n${bold}Wrong option${normal}";;
    esac
    done
}

# Initialization

main_menu