use anyhow::Result;
use colored::Colorize;

use crate::scripts;

#[derive(Debug, Clone, Copy)]
pub enum Installer {
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
}

impl Installer {
    pub fn execute(&self) -> Result<()> {
        println!();

        let result = match self {
            Installer::Install1Password => scripts::install_1password(),
            Installer::InstallZsh => scripts::install_zsh(),
            Installer::InstallVSCode => scripts::install_vscode(),
            Installer::InstallDocker => scripts::install_docker(),
            Installer::InstallRust => scripts::install_rust(),
            Installer::InstallGo => scripts::install_go(),
            Installer::InstallNvm => scripts::install_nvm(),
            Installer::InstallFonts => scripts::install_fonts(),
            Installer::InstallDiscord => scripts::install_discord(),
            Installer::EnableFractionalScaling => scripts::enable_fractional_scaling(),
            Installer::DisableFractionalScaling => scripts::disable_fractional_scaling(),
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
