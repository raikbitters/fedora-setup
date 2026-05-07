use anyhow::Result;
use colored::Colorize;

use crate::scripts;

#[derive(Debug, Clone, Copy)]
pub enum Installer {
    Install1Password,
    InstallZsh,
    InstallVSCode,
    InstallDocker,
    InstallFonts,
    InstallDiscord,
    SetupGit,
}

impl Installer {
    pub fn execute(&self) -> Result<()> {
        println!();

        let result = match self {
            Installer::Install1Password => scripts::install_1password(),
            Installer::InstallZsh => scripts::install_zsh(),
            Installer::InstallVSCode => scripts::install_vscode(),
            Installer::InstallDocker => scripts::install_docker(),
            Installer::InstallFonts => scripts::install_fonts(),
            Installer::InstallDiscord => scripts::install_discord(),
            Installer::SetupGit => scripts::setup_git(),
        };

        if let Err(e) = &result {
            eprintln!("{} {}", "Error:".red().bold(), e);
        }

        println!("\nPress Enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        result
    }
}
