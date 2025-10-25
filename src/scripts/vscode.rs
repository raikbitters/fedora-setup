use anyhow::Result;
use cmd_lib::run_cmd;
use colored::Colorize;

pub fn install_vscode() -> Result<()> {
    println!("{}", "Installing Visual Studio Code...".green().bold());

    // Import Microsoft GPG key
    let key_url = "https://packages.microsoft.com/keys/microsoft.asc";
    run_cmd!(sudo rpm --import $key_url)?;

    // Add repository
    let repo_content = r#"[code]
name=Visual Studio Code
baseurl=https://packages.microsoft.com/yumrepos/vscode
enabled=1
gpgcheck=1
gpgkey=https://packages.microsoft.com/keys/microsoft.asc"#;

    run_cmd!(echo $repo_content | sudo tee /etc/yum.repos.d/vscode.repo > /dev/null)?;

    // Update package lists
    run_cmd!(sudo dnf update -y)?;

    // Install VS Code
    run_cmd!(sudo dnf install -y code)?;

    println!("{}", "✓ VS Code installed successfully!".green().bold());
    Ok(())
}
