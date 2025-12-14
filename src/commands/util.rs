use std::{fs, path::Path};

use colored::Colorize;

use crate::models::{Command, HelpInfo};

pub struct ExitCommand;
impl Command for ExitCommand {
    fn execute(&self, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn info(&self) -> crate::models::HelpInfo {
        HelpInfo {
            label: "exit",
            aliases: &["q", "quit"],
            description: "Temp",
            usage: "Usage",
        }
    }
}

pub struct CleanCommand;
impl Command for CleanCommand {
    fn execute(&self, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        let output_dir = "./data";
        let db_file = "storage.db";

        println!("Cleaning project data...");

        // Delete ./data folder
        if Path::new(output_dir).exists() {
            fs::remove_dir_all(output_dir)?;
            println!("  ✓ Deleted '{}' directory", output_dir);
        } else {
            println!("  - '{}' does not exist (skipping)", output_dir);
        }

        // Delete the database file
        if Path::new(db_file).exists() {
            fs::remove_file(db_file)?;
            println!("  ✓ Deleted '{}'", db_file);
        }

        println!("Clean complete.");
        Ok(())
    }

    fn info(&self) -> crate::models::HelpInfo {
        HelpInfo {
            label: "clean",
            aliases: &["c", "cl"],
            description: "Temp",
            usage: "Usage",
        }
    }
}

use super::get_all_commands;
pub struct HelpCommand;
impl Command for HelpCommand {
    fn execute(&self, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}\n", "Rust-ETL CLI".blue().bold());

        let commands = get_all_commands();

        let cmd_specifier = args.get(1);
        if let Some(specifier) = cmd_specifier {
            println!("Help Info: {specifier}");
            for cmd in commands {
                let info = cmd.info();
                if info.label != *specifier {
                    continue;
                }

                println!(
                    "\n{:<12}\n\n{:<20}\n{}\n{}\n",
                    info.label.green().bold(),     // Command Label
                    format!("{:?}", info.aliases), // Aliases
                    info.description,
                    info.usage
                );
            }
        } else {
            for cmd in commands {
                let info = cmd.info();

                println!(
                    "  {:<12} {:<20} {}",
                    info.label.green().bold(),     // "generate"
                    format!("{:?}", info.aliases), // "['g', 'gen']"
                    info.description
                );
            }
            println!("\nType 'help <command>' for specific details.\n");
        }
        Ok(())
    }

    fn info(&self) -> crate::models::HelpInfo {
        HelpInfo {
            label: "help",
            aliases: &["h", "--help", "?"],
            description: "Temp",
            usage: "Usage",
        }
    }
}
