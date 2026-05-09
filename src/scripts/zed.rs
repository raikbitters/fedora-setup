use anyhow::Result;
use cmd_lib::run_cmd;
use colored::Colorize;

pub fn install_zed() -> Result<()> {
    println!("{}", "Installing Zed...".green().bold());

    run_cmd!(bash -c "curl -f https://zed.dev/install.sh | sh")?;

    Ok(())
}
