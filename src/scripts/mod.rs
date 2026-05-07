mod discord;
mod docker;
mod fonts;
mod git;
mod onepassword;
mod vscode;
mod zsh;

pub use discord::install_discord;
pub use docker::install_docker;
pub use fonts::install_fonts;
pub use git::setup_git;
pub use onepassword::install_1password;
pub use vscode::install_vscode;
pub use zsh::install_zsh;
