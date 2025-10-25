use anyhow::Result;
use cmd_lib::run_cmd;
use colored::Colorize;

pub fn install_docker() -> Result<()> {
    println!("{}", "Installing Docker...".green().bold());

    let user = std::env::var("USER")?;

    // Add Docker repository
    let repo_url = "https://download.docker.com/linux/fedora/docker-ce.repo";
    run_cmd!(sudo dnf config-manager addrepo $repo_url)?;

    // Install Docker
    run_cmd!(sudo dnf install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin)?;

    // Start and enable Docker
    run_cmd!(sudo systemctl enable --now docker)?;

    // Add user to docker group
    run_cmd!(sudo usermod -aG docker $user)?;

    println!("{}", "✓ Docker installed successfully!".green().bold());
    println!(
        "\n{} Please log out and log back in so that your group membership is re-evaluated.",
        "⚠".yellow().bold()
    );

    Ok(())
}
