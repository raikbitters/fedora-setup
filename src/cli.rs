use crate::scripts;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum CliCommand {
    Install1Password,
    InstallZsh,
    InstallMise,
    InstallVSCode,
    InstallZed,
    InstallDocker,
    InstallFonts,
    InstallDiscord,
    SetupGit,
}

impl CliCommand {
    pub fn name(&self) -> &'static str {
        match self {
            CliCommand::Install1Password => "1password",
            CliCommand::InstallZsh => "zsh",
            CliCommand::InstallMise => "mise",
            CliCommand::InstallVSCode => "vscode",
            CliCommand::InstallZed => "zed",
            CliCommand::InstallDocker => "docker",
            CliCommand::InstallFonts => "fonts",
            CliCommand::InstallDiscord => "discord",
            CliCommand::SetupGit => "git",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            CliCommand::Install1Password => "Install 1Password",
            CliCommand::InstallZsh => "Install Zsh with Oh My Zsh",
            CliCommand::InstallMise => "Install Mise",
            CliCommand::InstallVSCode => "Install Visual Studio Code",
            CliCommand::InstallZed => "Install Zed",
            CliCommand::InstallDocker => "Install Docker",
            CliCommand::InstallFonts => "Install additional fonts",
            CliCommand::InstallDiscord => "Install Discord",
            CliCommand::SetupGit => "Set up Git user name and email",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "1password" => Some(CliCommand::Install1Password),
            "zsh" => Some(CliCommand::InstallZsh),
            "mise" => Some(CliCommand::InstallMise),
            "vscode" => Some(CliCommand::InstallVSCode),
            "zed" => Some(CliCommand::InstallZed),
            "docker" => Some(CliCommand::InstallDocker),
            "fonts" => Some(CliCommand::InstallFonts),
            "discord" => Some(CliCommand::InstallDiscord),
            "git" => Some(CliCommand::SetupGit),
            _ => None,
        }
    }

    pub fn action(&self) -> fn() -> Result<()> {
        match self {
            CliCommand::Install1Password => scripts::install_1password,
            CliCommand::InstallZsh => scripts::install_zsh,
            CliCommand::InstallMise => scripts::install_mise,
            CliCommand::InstallVSCode => scripts::install_vscode,
            CliCommand::InstallZed => scripts::install_zed,
            CliCommand::InstallDocker => scripts::install_docker,
            CliCommand::InstallFonts => scripts::install_fonts,
            CliCommand::InstallDiscord => scripts::install_discord,
            CliCommand::SetupGit => scripts::setup_git,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            CliCommand::Install1Password,
            CliCommand::InstallZsh,
            CliCommand::InstallMise,
            CliCommand::InstallVSCode,
            CliCommand::InstallZed,
            CliCommand::InstallDocker,
            CliCommand::InstallFonts,
            CliCommand::InstallDiscord,
            CliCommand::SetupGit,
        ]
    }
}
