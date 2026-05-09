use crate::scripts;
use anyhow::Result;
use colored::Colorize;
use std::fmt;

#[derive(Debug, Clone)]
pub enum MainMenuItem {
    Separator(&'static str),
    Install1Password,
    InstallZsh,
    InstallMise,
    InstallVSCode,
    InstallZed,
    InstallDocker,
    InstallFonts,
    InstallDiscord,
    SetupGit,
    Exit,
}

impl MainMenuItem {
    pub fn action(&self) -> Option<fn() -> Result<()>> {
        match self {
            MainMenuItem::Install1Password => Some(scripts::install_1password),
            MainMenuItem::InstallZsh => Some(scripts::install_zsh),
            MainMenuItem::InstallMise => Some(scripts::install_mise),
            MainMenuItem::InstallVSCode => Some(scripts::install_vscode),
            MainMenuItem::InstallZed => Some(scripts::install_zed),
            MainMenuItem::InstallDocker => Some(scripts::install_docker),
            MainMenuItem::InstallFonts => Some(scripts::install_fonts),
            MainMenuItem::InstallDiscord => Some(scripts::install_discord),
            MainMenuItem::SetupGit => Some(scripts::setup_git),
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
            MainMenuItem::InstallMise => write!(f, "Install Mise"),
            MainMenuItem::InstallVSCode => write!(f, "Install Visual Studio Code"),
            MainMenuItem::InstallZed => write!(f, "Install Zed"),
            MainMenuItem::InstallDocker => write!(f, "Install Docker"),
            MainMenuItem::InstallFonts => write!(f, "Install additional fonts"),
            MainMenuItem::InstallDiscord => write!(f, "Install Discord"),
            MainMenuItem::SetupGit => write!(f, "Set up Git user name and email"),
            MainMenuItem::Exit => write!(f, "Exit"),
        }
    }
}

pub const MAIN_MENU_ITEMS: &[MainMenuItem] = &[
    MainMenuItem::Separator("Installation options:"),
    MainMenuItem::Install1Password,
    MainMenuItem::InstallZsh,
    MainMenuItem::InstallMise,
    MainMenuItem::InstallVSCode,
    MainMenuItem::InstallZed,
    MainMenuItem::InstallDocker,
    MainMenuItem::InstallFonts,
    MainMenuItem::InstallDiscord,
    MainMenuItem::Separator("Configuration options:"),
    MainMenuItem::SetupGit,
    MainMenuItem::Separator(""),
    MainMenuItem::Exit,
];

pub fn install(f: impl FnOnce() -> Result<()>) -> Result<()> {
    if let Err(e) = f() {
        eprintln!("{} {}", "Error:".red().bold(), e);
    }
    println!("\nPress Enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(())
}
