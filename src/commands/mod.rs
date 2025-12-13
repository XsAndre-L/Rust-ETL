use std::{env, io};

pub mod generate;
pub mod transform;

pub struct ParsedCommand {
    pub cmd: String,
    pub args: Vec<String>,
}

impl ParsedCommand {
    pub fn new(input: &str) -> Option<Self> {
        let mut parts = input.trim().split_whitespace();

        let cmd = parts.next().map(|s| s.to_string())?;

        let args = parts.map(|s| s.to_string()).collect();
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
        match *command {
            "exit" | "quit" => return false,

            "g" | "gen" | "generate" => {
                // Example of handling an optional arg: "gen csv" vs "gen ndjson"
                // If they just typed "gen", default to csv
                let format = args.get(1).unwrap_or(&"csv");
                println!("Generating {}...", format);

                match generate::generate() {
                    Ok(_) => println!("Generation completed successfully."),
                    Err(e) => println!("Failed to generate data: {}", e),
                }
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

            "reset" => {
                // this will delete ./data folder
                // delete ./target folder
                // delete the storage.db
            }

            "h" | "help" => {}

            _ => {
                println!("Unknown command: '{}'", command);
            }
        }
        return true;
    }

    true
}
