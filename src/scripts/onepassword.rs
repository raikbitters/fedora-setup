use anyhow::Result;
use cmd_lib::run_cmd;
use colored::Colorize;

pub fn install_1password() -> Result<()> {
    println!("{}", "Installing 1Password...".green().bold());

    // Import the 1Password GPG key
    let key_url = "https://downloads.1password.com/linux/keys/1password.asc";
    run_cmd!(sudo rpm --import $key_url)?;

    // Add the 1Password repository
    let repo_content = r#"[1password]
name=1Password Stable Channel
baseurl=https://downloads.1password.com/linux/rpm/stable/$basearch
enabled=1
gpgcheck=1
repo_gpgcheck=1
gpgkey="https://downloads.1password.com/linux/keys/1password.asc""#;

    run_cmd!(echo $repo_content | sudo tee /etc/yum.repos.d/1password.repo > /dev/null)?;

    // Update package lists
    run_cmd!(sudo dnf update -y)?;

    // Install 1Password
    run_cmd!(sudo dnf install -y 1password)?;

    println!("{}", "✓ 1Password installed successfully!".green().bold());
    Ok(())
}
