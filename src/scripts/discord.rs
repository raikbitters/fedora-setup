use anyhow::Result;
use cmd_lib::run_cmd;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

pub fn install_discord() -> Result<()> {
    println!("{}", "Installing Discord...".green().bold());

    // Get home directory
    let home = std::env::var("HOME")?;
    let install_dir = PathBuf::from(&home).join(".local/share");
    let discord_dir = install_dir.join("Discord");
    let applications_dir = PathBuf::from(&home).join(".local/share/applications");

    // Download Discord
    println!("Downloading Discord...");
    run_cmd!(wget "https://discord.com/api/download/stable?platform=linux&format=tar.gz" -O /tmp/discord.tar.gz)?;

    // Remove old installation if exists
    if discord_dir.exists() {
        println!("Removing old Discord installation...");
        fs::remove_dir_all(&discord_dir)?;
    }

    // Extract Discord
    let install_path = install_dir.to_str().unwrap();
    run_cmd!(tar -xzf /tmp/discord.tar.gz -C $install_path)?;

    // Create desktop entry
    let desktop_entry = format!(
        r#"[Desktop Entry]
Name=Discord
StartupWMClass=discord
Comment=All-in-one voice and text chat for gamers that's free, secure, and works on both your desktop and phone.
GenericName=Internet Messenger
Exec={}
Icon={}
Type=Application
Categories=Network;InstantMessaging;
Path=/usr/bin"#,
        discord_dir.join("Discord").display(),
        discord_dir.join("discord.png").display()
    );

    let desktop_file = applications_dir.join("discord.desktop");
    fs::write(&desktop_file, desktop_entry)?;

    // Clean up
    run_cmd!(rm /tmp/discord.tar.gz)?;

    println!("{}", "✓ Discord installed successfully!".green().bold());
    println!(
        "{}",
        "You can launch it from the Applications menu.".green()
    );
    Ok(())
}
