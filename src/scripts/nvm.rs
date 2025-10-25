use anyhow::Result;
use colored::Colorize;

use super::utils::run_cmd;

pub fn install_nvm() -> Result<()> {
    println!("{}", "Installing Node Version Manager...".green().bold());

    let url = "https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh";
    run_cmd!(curl -o- $url | bash)?;

    println!("{}", "✓ NVM installed successfully!".green().bold());
    Ok(())
}
