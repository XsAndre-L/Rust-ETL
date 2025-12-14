use std::{
    env,
    io::{self, Write},
};

use crate::commands::execute_command;

mod commands;
mod db;
pub mod models;

// use std::prelude::rust_2024;

fn main() {
    let mut cmd = String::new();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Immediate Mode: Executes and exits
        cmd = args[1..].join(" ");
        execute_command(&mut cmd);
    } else {
        // Interactive Mode: Executes untill "exit" is called
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            cmd.clear();

            match io::stdin().read_line(&mut cmd) {
                Ok(_) => {
                    let exit = execute_command(&mut cmd);
                    if exit {
                        break;
                    }
                }
                Err(error) => println!("Error: {}", error),
            }
        }
    }
}
