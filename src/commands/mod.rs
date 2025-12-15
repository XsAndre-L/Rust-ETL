use crate::core::types::Command;

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
    pub label: String,
    pub args: Vec<String>,
}
impl ParsedCommand {
    pub fn new(input: &str) -> Option<Self> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            return None;
        }

        let label = parts[0].to_string();
        let args = parts[1..].iter().map(|s| s.to_string()).collect();

        Some(Self { label, args })
    }
}

pub fn execute_command(parsed_command: &mut ParsedCommand) -> bool {
    let command = &parsed_command.label;
    let commands = get_all_commands();

    for cmd in commands {
        let info = cmd.info();
        if info.label == command || info.aliases.contains(&command.as_str()) {
            if info.label == "exit" {
                return true;
            }

            let args_refs: Vec<&str> = parsed_command.args.iter().map(|s| s.as_str()).collect();

            if let Err(e) = cmd.execute(&args_refs) {
                println!("Error executing command: {e}");
            }
            return false;
        }
    }

    println!("Unknown command: '{command}'");

    false
}
