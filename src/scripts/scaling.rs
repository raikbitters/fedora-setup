use anyhow::Result;
use cmd_lib::run_cmd;
use colored::Colorize;

pub fn enable_fractional_scaling() -> Result<()> {
    println!("{}", "Enabling fractional scaling...".green().bold());

    run_cmd!(gsettings set org.gnome.mutter experimental-features "['scale-monitor-framebuffer']")?;

    println!("{}", "✓ Fractional scaling enabled!".green().bold());
    Ok(())
}

pub fn disable_fractional_scaling() -> Result<()> {
    println!("{}", "Disabling fractional scaling...".red().bold());

    run_cmd!(gsettings set org.gnome.mutter experimental-features "[]")?;

    println!("{}", "✓ Fractional scaling disabled!".red().bold());
    Ok(())
}
