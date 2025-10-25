use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};

mod cli;
mod installer;
mod menu;
mod scripts;

use menu::{MainMenuItem, MAIN_MENU_ITEMS};

#[derive(Parser)]
#[command(name = "fedora-setup")]
#[command(about = "Fast Fedora setup tool", long_about = None)]
#[command(version)]
struct Cli {
    command: Option<String>,

    #[arg(short, long)]
    list: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.list {
        println!("Available commands:");
        for cmd in cli::CliCommand::all() {
            println!("  {:20} - {}", cmd.name(), cmd.description());
        }
        return Ok(());
    }

    if let Some(cmd) = cli.command {
        return execute_command(&cmd);
    }

    run_interactive_menu()
}

fn execute_command(cmd: &str) -> Result<()> {
    match cli::CliCommand::from_name(cmd) {
        Some(cli_cmd) => {
            let installer = cli_cmd.installer();
            installer.execute()?;
            println!("\n{} completed successfully!", cmd.green());
        }
        None => {
            eprintln!("{}: Unknown command '{}'", "Error".red(), cmd);
            eprintln!("\nAvailable commands:");
            for cli_cmd in cli::CliCommand::all() {
                eprintln!("  - {}", cli_cmd.name());
            }
            std::process::exit(1);
        }
    }
    Ok(())
}

fn run_interactive_menu() -> Result<()> {
    loop {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");

        let user = std::env::var("USER").unwrap_or_else(|_| "User".to_string());
        let version = env!("CARGO_PKG_VERSION");

        println!("\nHello {}! It's fast Fedora setup {}.\n", user.cyan(), format!("v{}", version).bright_black());

        let menu_items: Vec<String> = MAIN_MENU_ITEMS
            .iter()
            .map(|item| {
                let prefix = match item {
                    MainMenuItem::Separator(text) => format!("\n{}", text.bold()),
                    MainMenuItem::Exit => String::new(),
                    _ => String::from("  "),
                };
                format!("{}{}", prefix, item)
            })
            .collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select option")
            .items(&menu_items)
            .default(0)
            .interact_opt()?;

        match selection {
            Some(index) => {
                let selected_item = &MAIN_MENU_ITEMS[index];

                match selected_item {
                    MainMenuItem::Exit => {
                        println!("\nGoodbye!");
                        break;
                    }
                    MainMenuItem::Separator(_) => { continue; }
                    _ => {
                        if let Some(installer) = selected_item.installer() {
                            let _ = installer.execute();
                        }
                    }
                }
            }
            None => {
                println!("\nGoodbye!");
                break;
            }
        }
    }

    Ok(())
}
