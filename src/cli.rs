use crate::installer::Installer;

#[derive(Debug, Clone)]
pub enum CliCommand {
    Install1Password,
    InstallZsh,
    InstallVSCode,
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
            CliCommand::InstallVSCode => "vscode",
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
            CliCommand::InstallVSCode => "Install Visual Studio Code",
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
            "vscode" => Some(CliCommand::InstallVSCode),
            "docker" => Some(CliCommand::InstallDocker),
            "fonts" => Some(CliCommand::InstallFonts),
            "discord" => Some(CliCommand::InstallDiscord),
            "git" => Some(CliCommand::SetupGit),
            _ => None,
        }
    }

    pub fn installer(&self) -> Installer {
        match self {
            CliCommand::Install1Password => Installer::Install1Password,
            CliCommand::InstallZsh => Installer::InstallZsh,
            CliCommand::InstallVSCode => Installer::InstallVSCode,
            CliCommand::InstallDocker => Installer::InstallDocker,
            CliCommand::InstallFonts => Installer::InstallFonts,
            CliCommand::InstallDiscord => Installer::InstallDiscord,
            CliCommand::SetupGit => Installer::SetupGit,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            CliCommand::Install1Password,
            CliCommand::InstallZsh,
            CliCommand::InstallVSCode,
            CliCommand::InstallDocker,
            CliCommand::InstallFonts,
            CliCommand::InstallDiscord,
            CliCommand::SetupGit,
        ]
    }
}
