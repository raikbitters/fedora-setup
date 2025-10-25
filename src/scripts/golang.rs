use anyhow::{Context, Result};
use cmd_lib::run_cmd;
use colored::Colorize;
use dialoguer::Input;

pub fn install_go() -> Result<()> {
    println!("{}", "Installing Go...".green().bold());

    let default_version = "1.25.3";
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
        let path_str = path.to_str().unwrap();

        // Add PATH for Go bin if not present
        let cmd1 = format!(r#"grep -qxF 'export PATH=$PATH:/usr/local/go/bin' {} || echo 'export PATH=$PATH:/usr/local/go/bin' >> {}"#, path_str, path_str);
        run_cmd!(sh -c $cmd1)?;

        // Add GOPATH if not present
        let cmd2 = format!(r#"grep -qxF 'export GOPATH=$HOME/.go' {} || echo 'export GOPATH=$HOME/.go' >> {}"#, path_str, path_str);
        run_cmd!(sh -c $cmd2)?;

        // Add PATH for GOPATH bin if not present
        let cmd3 = format!(r#"grep -qxF 'export PATH=$PATH:$GOPATH/bin' {} || echo 'export PATH=$PATH:$GOPATH/bin' >> {}"#, path_str, path_str);
        run_cmd!(sh -c $cmd3)?;
    }

    // Clean up
    run_cmd!(rm $filename)?;

    println!("{}", "✓ Go installed successfully!".green().bold());
    Ok(())
}
