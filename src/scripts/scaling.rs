use anyhow::Result;
use colored::Colorize;

use super::utils::run_cmd;

pub fn enable_fractional_scaling() -> Result<()> {
    println!("{}", "Enabling fractional scaling...".green().bold());

    run_cmd!(gsettings set org.gnome.mutter experimental-features "['scale-monitor-framebuffer']")?;

    println!("{}", "✓ Fractional scaling enabled!".green().bold());
    Ok(())
}
