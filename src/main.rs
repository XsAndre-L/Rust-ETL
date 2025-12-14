use std::{
    env,
    io::{self, Write},
};

use crate::commands::{ParsedCommand, execute_command};

mod commands;
mod db;
pub mod models;

// use std::prelude::rust_2024;

fn main() {
    let mut cmd = String::new();
    let mut parsed_cmd: ParsedCommand;

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Immediate Mode: Executes and exits
        cmd = args[1..].join(" ");
        if let Some(mut parsed_cmd) = ParsedCommand::new(&cmd) {
            execute_command(&mut parsed_cmd);
        }
        // parsed_cmd.label = args[0].clone();
        // parsed_cmd.args = args[1..].to_vec();
        // execute_command(&mut parsed_cmd);
    } else {
        // Interactive Mode: Executes untill "exit" is called
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            cmd.clear();

            match io::stdin().read_line(&mut cmd) {
                Ok(_) => {
                    if let Some(mut parsed_cmd) = ParsedCommand::new(&cmd) {
                        let exit = execute_command(&mut parsed_cmd);
                        if exit {
                            break;
                        }
                    }
                    // parsed_cmd = ParsedCommand::new(cmd);
                    // let exit = execute_command(&mut cmd);
                    // if exit {
                    //     break;
                    // }
                }
                Err(error) => println!("Error: {}", error),
            }
        }
    }
}
