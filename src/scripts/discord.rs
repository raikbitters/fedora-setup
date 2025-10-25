use anyhow::Result;
use colored::Colorize;
use std::fs;

use super::utils::run_cmd;

pub fn install_discord() -> Result<()> {
    println!("{}", "Installing Discord...".green().bold());

    // Download Discord
    println!("Downloading Discord...");
    run_cmd!(curl -L -o /tmp/discord.tar.gz "https://discord.com/api/download?platform=linux&format=tar.gz")?;

    // Extract to /opt
    run_cmd!(sudo tar -xzf /tmp/discord.tar.gz -C /opt)?;

    // Create symlink
    run_cmd!(sudo ln -sf /opt/Discord/Discord /usr/bin/Discord)?;

    // Create desktop entry
    let desktop_entry = r#"[Desktop Entry]
Name=Discord
StartupWMClass=discord
Comment=All-in-one voice and text chat for gamers
GenericName=Internet Messenger
Exec=/usr/bin/Discord
Icon=/opt/Discord/discord.png
Type=Application
Categories=Network;InstantMessaging;
Path=/usr/bin"#;

    fs::write("/tmp/discord.desktop", desktop_entry)?;
    run_cmd!(sudo mv /tmp/discord.desktop /usr/share/applications/discord.desktop)?;

    // Clean up
    run_cmd!(rm /tmp/discord.tar.gz)?;

    println!("{}", "✓ Discord installed successfully!".green().bold());
    Ok(())
}
