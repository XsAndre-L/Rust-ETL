use std::{
    env,
    io::{self, Write},
};

use crate::commands::{execute_command, generate, transform};

mod commands;
mod db;
pub mod models;

// use std::prelude::rust_2024;

fn main() {
    let mut cmd = String::new();
    let args: Vec<String> = env::args().collect();

    // io::stdin().read_line(&mut cmd);

    // let parts = env::args();

    if args.len() > 1 {
        println!("Immediate: {}", args.len());
        cmd = args[1..].join(" ");
        execute_command(&mut cmd);
    } else {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            cmd.clear();

            // println!("Interactive: {}", args.len());

            match io::stdin().read_line(&mut cmd) {
                Ok(_) => {
                    if !cmd.is_empty() {
                        let repeat = execute_command(&mut cmd);
                        if !repeat {
                            break;
                        }
                    }

                    // if !execute_command(&mut cmd) {
                    //     break;
                    // }
                }
                Err(error) => println!("Error: {}", error),
            }
        }
    }
}
