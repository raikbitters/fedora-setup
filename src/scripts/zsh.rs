use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;

use super::utils::run_cmd;

pub fn install_zsh() -> Result<()> {
    println!("{}", "Installing Zsh with Oh My Zsh...".green().bold());

    // Install Zsh and utilities
    run_cmd!(sudo dnf install -y zsh util-linux-user)?;

    // Install Oh My Zsh
    let home = dirs::home_dir().context("Could not find home directory")?;
    let oh_my_zsh = home.join(".oh-my-zsh");

    if !oh_my_zsh.exists() {
        let url = "https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh";
        run_cmd!(sh -c "$(curl -fsSL $url)" "" --unattended)?;
    }

    // Install plugins
    let custom_plugins = oh_my_zsh.join("custom/plugins");

    // zsh-autosuggestions
    let autosuggestions = custom_plugins.join("zsh-autosuggestions");
    if !autosuggestions.exists() {
        let path = autosuggestions.to_str().unwrap();
        let url = "https://github.com/zsh-users/zsh-autosuggestions";
        run_cmd!(git clone $url $path)?;
    }

    // zsh-syntax-highlighting
    let syntax_highlighting = custom_plugins.join("zsh-syntax-highlighting");
    if !syntax_highlighting.exists() {
        let path = syntax_highlighting.to_str().unwrap();
        let url = "https://github.com/zsh-users/zsh-syntax-highlighting.git";
        run_cmd!(git clone $url $path)?;
    }

    // k plugin (directory listings)
    let k_plugin = custom_plugins.join("k");
    if !k_plugin.exists() {
        let path = k_plugin.to_str().unwrap();
        let url = "https://github.com/supercrabtree/k";
        run_cmd!(git clone $url $path)?;
    }

    // Copy zsh configuration
    let zshrc_source = include_str!("../../assets/zsh/.zshrc");
    let zshrc_dest = home.join(".zshrc");
    fs::write(&zshrc_dest, zshrc_source)?;

    // GTK CSS for increased padding
    let gtk_config_dir = home.join(".config/gtk-4.0");
    fs::create_dir_all(&gtk_config_dir)?;

    let gtk_css_source = include_str!("../../assets/zsh/gtk.css");
    let gtk_css_dest = gtk_config_dir.join("gtk.css");
    fs::write(&gtk_css_dest, gtk_css_source)?;

    // Set Zsh as default shell
    let user = std::env::var("USER")?;
    run_cmd!(sudo chsh -s /bin/zsh $user)?;

    println!("{}", "✓ Zsh installed successfully!".green().bold());
    Ok(())
}
