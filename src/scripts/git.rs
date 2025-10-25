use anyhow::Result;
use cmd_lib::run_cmd;
use colored::Colorize;
use dialoguer::Input;

pub fn setup_git() -> Result<()> {
    println!("{}", "Setting up Git...".green().bold());

    let name: String = Input::new()
        .with_prompt("Enter your Git user name")
        .interact_text()?;

    let email: String = Input::new()
        .with_prompt("Enter your Git user email")
        .interact_text()?;

    run_cmd!(git config --global user.name $name)?;
    run_cmd!(git config --global user.email $email)?;

    println!("{}", "✓ Git configured successfully!".green().bold());
    Ok(())
}
