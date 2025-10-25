use anyhow::Result;
use cmd_lib::run_cmd;
use colored::Colorize;

pub fn install_nvm() -> Result<()> {
    println!("{}", "Installing Node Version Manager...".green().bold());

    let url = "https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh";
    run_cmd!(curl -o- $url | bash)?;

    println!("{}", "✓ NVM installed successfully!".green().bold());
    Ok(())
}
