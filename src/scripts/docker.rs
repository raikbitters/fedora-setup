use anyhow::Result;
use colored::Colorize;

use super::utils::run_cmd;

pub fn install_docker() -> Result<()> {
    println!("{}", "Installing Docker...".green().bold());

    let user = std::env::var("USER")?;

    // Add Docker repository
    let repo_url = "https://download.docker.com/linux/fedora/docker-ce.repo";
    run_cmd!(sudo dnf config-manager addrepo $repo_url)?;

    // Install Docker
    run_cmd!(sudo dnf install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin)?;

    // Start and enable Docker
    run_cmd!(sudo systemctl start docker)?;
    run_cmd!(sudo systemctl enable docker.service)?;
    run_cmd!(sudo systemctl enable containerd.service)?;

    // Add user to docker group
    run_cmd!(sudo usermod -aG docker $user)?;

    println!("{}", "✓ Docker installed successfully!".green().bold());
    println!(
        "\n{} Please log out and log back in so that your group membership is re-evaluated.",
        "⚠".yellow().bold()
    );

    Ok(())
}
