use crate::installer::Installer;

#[derive(Debug, Clone)]
pub enum CliCommand {
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

impl CliCommand {
    pub fn name(&self) -> &'static str {
        match self {
            CliCommand::Install1Password => "1password",
            CliCommand::InstallZsh => "zsh",
            CliCommand::InstallVSCode => "vscode",
            CliCommand::InstallDocker => "docker",
            CliCommand::InstallRust => "rust",
            CliCommand::InstallGo => "go",
            CliCommand::InstallNvm => "nvm",
            CliCommand::InstallFonts => "fonts",
            CliCommand::InstallDiscord => "discord",
            CliCommand::EnableFractionalScaling => "enable-scaling",
            CliCommand::DisableFractionalScaling => "disable-scaling",
            CliCommand::SetupGit => "git",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            CliCommand::Install1Password => "Install 1Password",
            CliCommand::InstallZsh => "Install Zsh with Oh My Zsh",
            CliCommand::InstallVSCode => "Install Visual Studio Code",
            CliCommand::InstallDocker => "Install Docker",
            CliCommand::InstallRust => "Install Rust",
            CliCommand::InstallGo => "Install Go",
            CliCommand::InstallNvm => "Install Node Version Manager",
            CliCommand::InstallFonts => "Install additional fonts",
            CliCommand::InstallDiscord => "Install Discord",
            CliCommand::EnableFractionalScaling => "Enable fractional scaling",
            CliCommand::DisableFractionalScaling => "Disable fractional scaling",
            CliCommand::SetupGit => "Set up Git user name and email",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "1password" => Some(CliCommand::Install1Password),
            "zsh" => Some(CliCommand::InstallZsh),
            "vscode" => Some(CliCommand::InstallVSCode),
            "docker" => Some(CliCommand::InstallDocker),
            "rust" => Some(CliCommand::InstallRust),
            "go" => Some(CliCommand::InstallGo),
            "nvm" => Some(CliCommand::InstallNvm),
            "fonts" => Some(CliCommand::InstallFonts),
            "discord" => Some(CliCommand::InstallDiscord),
            "enable-scaling" => Some(CliCommand::EnableFractionalScaling),
            "disable-scaling" => Some(CliCommand::DisableFractionalScaling),
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
            CliCommand::InstallRust => Installer::InstallRust,
            CliCommand::InstallGo => Installer::InstallGo,
            CliCommand::InstallNvm => Installer::InstallNvm,
            CliCommand::InstallFonts => Installer::InstallFonts,
            CliCommand::InstallDiscord => Installer::InstallDiscord,
            CliCommand::EnableFractionalScaling => Installer::EnableFractionalScaling,
            CliCommand::DisableFractionalScaling => Installer::DisableFractionalScaling,
            CliCommand::SetupGit => Installer::SetupGit,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            CliCommand::Install1Password,
            CliCommand::InstallZsh,
            CliCommand::InstallVSCode,
            CliCommand::InstallDocker,
            CliCommand::InstallRust,
            CliCommand::InstallGo,
            CliCommand::InstallNvm,
            CliCommand::InstallFonts,
            CliCommand::InstallDiscord,
            CliCommand::EnableFractionalScaling,
            CliCommand::DisableFractionalScaling,
            CliCommand::SetupGit,
        ]
    }
}
