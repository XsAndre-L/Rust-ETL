use std::{env, io};

use colored::Colorize;

use crate::commands::{
    generate::GenerateCommand,
    transform::TransformCommand,
    util::{CleanCommand, ExitCommand},
};

use crate::models::Command;

pub mod clean;
pub mod generate;
pub mod transform;
pub mod util;

pub fn get_all_commands() -> Vec<Box<dyn Command>> {
    vec![
        Box::new(generate::GenerateCommand),
        Box::new(transform::TransformCommand),
        Box::new(util::CleanCommand),
        Box::new(util::HelpCommand),
        Box::new(util::ExitCommand),
    ]
}

pub struct ParsedCommand {
    pub cmd: String,
    pub args: Vec<String>,
}

impl ParsedCommand {
    pub fn new(input: &str) -> Option<Self> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            return None;
        }
        // let mut parts = input.trim().split_whitespace();

        // let cmd = parts.next().map(|s| s.to_string())?;
        let cmd = parts[0].to_string();
        let args = parts[1..].iter().map(|s| s.to_string()).collect();

        // let args = parts.map(|s| s.to_string()).collect();
        Some(Self { cmd, args })
    }

    pub fn from_env() -> Option<Self> {
        let mut parts = env::args();
        parts.next(); // Skip executable

        let cmd = parts.next()?;
        let args: Vec<String> = parts.collect();

        Some(Self { cmd, args })
    }
}

pub fn execute_command(cmd: &mut String) -> bool {
    let args: Vec<&str> = cmd.trim().split_whitespace().collect();

    if let Some(command) = args.first() {
        let commands = get_all_commands();

        for cmd in commands {
            let info = cmd.info();
            if info.name == *command || info.aliases.contains(command) {
                if info.name == "exit" {
                    return true;
                }

                if let Err(e) = cmd.execute(&args) {
                    println!("Error executing command: {}", e);
                }
                return false;
            }
        }

        println!("Unknown command: '{}'", command);

        // match *command {
        //     "exit" | "quit" => return true,

        //     "g" | "gen" | "generate" => match GenerateCommand.execute(&args[1..]) {
        //         Ok(_) => println!("Generation completed successfully."),
        //         Err(e) => println!("Failed to generate data: {}", e),
        //     },

        //     "t" | "transform" => {
        //         match TransformCommand.execute(&args[1..]) {
        //             Ok(_) => println!("Success!"),
        //             Err(e) => eprintln!("Error: {}", e),
        //         }

        //         // // Check for the required argument (the filename)
        //         // if let Some(filename) = args.get(1) {
        //         //     println!("Transforming file: {}", filename);

        //         //     // Pass the filename to your transform module
        //         //     match TransformCommand.execute(filename) {
        //         //         Ok(_) => println!("Success!"),
        //         //         Err(e) => eprintln!("Error: {}", e),
        //         //     }
        //         // } else {
        //         //     println!("Usage: transform <filename>");
        //         // }
        //     }

        //     "reset" => {
        //         // this will delete ./data folder
        //         // delete ./target folder
        //         // delete the storage.db
        //     }

        //     "h" | "help" => {
        //         println!("\n{}\n", "Rust-ETL CLI".blue().bold());

        //         let commands = get_all_commands();

        //         let cmd_specifier = args.get(1);
        //         if let Some(specifier) = cmd_specifier {
        //             println!("Help Info: {}", specifier);
        //             for cmd in commands {
        //                 let info = cmd.info();
        //                 if info.name != *specifier {
        //                     continue;
        //                 }

        //                 println!(
        //                     "\n{:<12}\n\n{:<20}\n{}\n{}\n",
        //                     info.name.green().bold(),      // "generate"
        //                     format!("{:?}", info.aliases), // "['g', 'gen']"
        //                     info.description,              // "Generates dummy data..."
        //                     info.usage
        //                 );
        //             }
        //         } else {
        //             for cmd in commands {
        //                 let info = cmd.info();

        //                 println!(
        //                     "  {:<12} {:<20} {}",
        //                     info.name.green().bold(),      // "generate"
        //                     format!("{:?}", info.aliases), // "['g', 'gen']"
        //                     info.description               // "Generates dummy data..."
        //                 );
        //             }
        //             println!("\nType 'help <command>' for specific details.\n");
        //         }
        //     }

        //     _ => {
        //         println!("Unknown command: '{}'", command);
        //     }
        // }
        return false;
    }

    false
}
