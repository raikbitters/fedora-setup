use anyhow::Result;
use cmd_lib::run_cmd;
use colored::Colorize;

pub fn install_fonts() -> Result<()> {
    println!("{}", "Installing additional fonts...".green().bold());

    // Fira Code for VS Code
    run_cmd!(sudo dnf install -y fira-code-fonts)?;

    // Font Awesome for Oh My Zsh
    run_cmd!(sudo dnf install -y fontawesome-fonts-all)?;

    // Powerline fonts
    run_cmd!(pip install --user powerline-status)?;

    println!("{}", "✓ Fonts installed successfully!".green().bold());
    Ok(())
}
