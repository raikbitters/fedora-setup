use anyhow::Result;
use cmd_lib::run_cmd;
use colored::Colorize;

pub fn install_rust() -> Result<()> {
    println!("{}", "Installing Rust...".green().bold());

    let url = "https://sh.rustup.rs";
    run_cmd!(curl --proto "=https" --tlsv1.2 -sSf $url | sh -s -- -y)?;

    println!("{}", "✓ Rust installed successfully!".green().bold());
    Ok(())
}
