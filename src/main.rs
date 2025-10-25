use anyhow::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};

mod installer;
mod menu;
mod scripts;

use menu::{MainMenuItem, MAIN_MENU_ITEMS};

fn main() -> Result<()> {
    loop {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");

        // Get current user
        let user = std::env::var("USER").unwrap_or_else(|_| "User".to_string());

        println!("\nHello {}! It's fast Fedora setup.\n", user.cyan());

        // Prepare menu items for display
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

        // Show menu and get selection
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
                    MainMenuItem::Separator(_) => {
                        // Separators are not selectable, continue loop
                        continue;
                    }
                    _ => {
                        if let Some(installer) = selected_item.installer() {
                            let _ = installer.execute();
                        }
                    }
                }
            }
            None => {
                // User cancelled (Ctrl+C)
                println!("\nGoodbye!");
                break;
            }
        }
    }

    Ok(())
}
