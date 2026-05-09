use anyhow::{Context, Result};
use cmd_lib::run_cmd;
use colored::Colorize;

pub fn install_mise() -> Result<()> {
    println!("{}", "Installing Mise...".green().bold());

    run_cmd!(bash -c "curl https://mise.run | sh")?;

    let home = dirs::home_dir().context("Could not find home directory")?;
    let oh_my_zsh = home.join(".oh-my-zsh");
    if oh_my_zsh.exists() {
        run_cmd!(bash -c "echo 'eval \"$(~/.local/bin/mise activate zsh)\"' >> ~/.zshrc")?;
        run_cmd!(bash -c "echo 'eval \"$(mise activate zsh --shims)\"' >> ~/.zprofile")?;

        run_cmd!(bash -c "~/.local/bin/mise use -g usage")?;
        run_cmd!(bash -c "mkdir -p ~/.oh-my-zsh/custom/completions")?;
        run_cmd!(bash -c "mise completion zsh > ~/.oh-my-zsh/custom/completions/_mise")?;

        run_cmd!(bash -c "rm -f ~/.zcompdump*")?;
        println!("{}", "Restart your shell or run: source ~/.zshrc".yellow());
    }

    run_cmd!(bash -c "~/.local/bin/mise doctor")?;
    Ok(())
}
