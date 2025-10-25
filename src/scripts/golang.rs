use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::Input;

use super::utils::{append_if_missing, run_cmd};

pub fn install_go() -> Result<()> {
    println!("{}", "Installing Go...".green().bold());

    let default_version = "1.23.2";
    let version: String = Input::new()
        .with_prompt("Enter Go version")
        .default(default_version.to_string())
        .interact_text()?;

    // Download Go
    let url = format!("https://golang.org/dl/go{}.linux-amd64.tar.gz", version);
    let filename = format!("go{}.linux-amd64.tar.gz", version);

    println!("Downloading Go {}...", version);
    run_cmd!(wget $url)?;

    // Remove old installation and extract
    run_cmd!(sudo rm -rf /usr/local/go)?;
    run_cmd!(sudo tar -C /usr/local -xzf $filename)?;

    // Set up environment variables
    let home = dirs::home_dir().context("Could not find home directory")?;

    let paths_to_update = vec![
        home.join(".profile"),
        home.join(".bash_profile"),
        home.join(".zshenv"),
    ];

    for path in paths_to_update {
        append_if_missing(&path, "export PATH=$PATH:/usr/local/go/bin")?;
        append_if_missing(&path, "export GOPATH=$HOME/.go")?;
        append_if_missing(&path, "export PATH=$PATH:$GOPATH/bin")?;
    }

    // Clean up
    run_cmd!(rm $filename)?;

    println!("{}", "✓ Go installed successfully!".green().bold());
    Ok(())
}
