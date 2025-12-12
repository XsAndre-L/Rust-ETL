use std::io::{self, Write};

use crate::commands::{generate, transform};

mod commands;
mod db;
pub mod models;

fn main() {
    let mut cmd = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        cmd.clear();

        match io::stdin().read_line(&mut cmd) {
            Ok(_) => {
                let args: Vec<&str> = cmd.trim().split_whitespace().collect();

                if let Some(command) = args.first() {
                    match *command {
                        "exit" | "quit" => break,

                        "g" | "gen" | "generate" => {
                            // Example of handling an optional arg: "gen csv" vs "gen ndjson"
                            // If they just typed "gen", default to csv
                            let format = args.get(1).unwrap_or(&"csv");
                            println!("Generating {}...", format);

                            // execute_generate(format);
                        }

                        "t" | "transform" => {
                            // Check for the required argument (the filename)
                            if let Some(filename) = args.get(1) {
                                println!("Transforming file: {}", filename);

                                // Pass the filename to your transform module
                                match transform::execute(filename) {
                                    Ok(_) => println!("Success!"),
                                    Err(e) => eprintln!("Error: {}", e),
                                }
                            } else {
                                println!("Usage: transform <filename>");
                            }
                        }

                        _ => println!("Unknown command: '{}'", command),
                    }
                }

                // Trim the cmd
                let command = cmd.trim();

                if command == "exit" {
                    break;
                }

                if command == "gen" || command == "generate" {
                    match generate::generate() {
                        Ok(_) => println!("Generation completed successfully."),
                        Err(e) => println!("Failed to generate data: {}", e),
                    }
                }

                println!("You typed: {}", command);
                // process_command(command); // Your logic goes here
            }
            Err(error) => {
                println!("Error reading cmd: {}", error);
                break;
            }
        }
    }
}
