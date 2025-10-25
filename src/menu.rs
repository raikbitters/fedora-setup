use std::fmt;

use crate::installer::Installer;

#[derive(Debug, Clone)]
pub enum MainMenuItem {
    Separator(&'static str),
    Install1Password,
    InstallZsh,
    InstallVSCode,
    InstallDocker,
    InstallRust,
    InstallGo,
    InstallNvm,
    InstallFonts,
    InstallDiscord,
    EnableFractionalScaling,
    DisableFractionalScaling,
    SetupGit,
    Exit,
}

impl MainMenuItem {
    pub fn installer(&self) -> Option<Installer> {
        match self {
            MainMenuItem::Install1Password => Some(Installer::Install1Password),
            MainMenuItem::InstallZsh => Some(Installer::InstallZsh),
            MainMenuItem::InstallVSCode => Some(Installer::InstallVSCode),
            MainMenuItem::InstallDocker => Some(Installer::InstallDocker),
            MainMenuItem::InstallRust => Some(Installer::InstallRust),
            MainMenuItem::InstallGo => Some(Installer::InstallGo),
            MainMenuItem::InstallNvm => Some(Installer::InstallNvm),
            MainMenuItem::InstallFonts => Some(Installer::InstallFonts),
            MainMenuItem::InstallDiscord => Some(Installer::InstallDiscord),
            MainMenuItem::EnableFractionalScaling => Some(Installer::EnableFractionalScaling),
            MainMenuItem::DisableFractionalScaling => Some(Installer::DisableFractionalScaling),
            MainMenuItem::SetupGit => Some(Installer::SetupGit),
            _ => None,
        }
    }
}

impl fmt::Display for MainMenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MainMenuItem::Separator(text) => write!(f, "{}", text),
            MainMenuItem::Install1Password => write!(f, "Install 1Password"),
            MainMenuItem::InstallZsh => write!(f, "Install Zsh with Oh My Zsh"),
            MainMenuItem::InstallVSCode => write!(f, "Install Visual Studio Code"),
            MainMenuItem::InstallDocker => write!(f, "Install Docker"),
            MainMenuItem::InstallRust => write!(f, "Install Rust"),
            MainMenuItem::InstallGo => write!(f, "Install Go"),
            MainMenuItem::InstallNvm => write!(f, "Install Node Version Manager"),
            MainMenuItem::InstallFonts => write!(f, "Install additional fonts"),
            MainMenuItem::InstallDiscord => write!(f, "Install Discord"),
            MainMenuItem::EnableFractionalScaling => write!(f, "Enable fractional scaling"),
            MainMenuItem::DisableFractionalScaling => write!(f, "Disable fractional scaling"),
            MainMenuItem::SetupGit => write!(f, "Set up Git user name and email"),
            MainMenuItem::Exit => write!(f, "Exit"),
        }
    }
}

pub const MAIN_MENU_ITEMS: &[MainMenuItem] = &[
    MainMenuItem::Separator("Installation options:"),
    MainMenuItem::Install1Password,
    MainMenuItem::InstallZsh,
    MainMenuItem::InstallVSCode,
    MainMenuItem::InstallDocker,
    MainMenuItem::InstallRust,
    MainMenuItem::InstallGo,
    MainMenuItem::InstallNvm,
    MainMenuItem::InstallFonts,
    MainMenuItem::InstallDiscord,
    MainMenuItem::Separator("Configuration options:"),
    MainMenuItem::EnableFractionalScaling,
    MainMenuItem::DisableFractionalScaling,
    MainMenuItem::SetupGit,
    MainMenuItem::Separator(""),
    MainMenuItem::Exit,
];
