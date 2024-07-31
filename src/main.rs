use PebbleVault;
use serde::Deserialize;
use serde_json::Value;
use std::io::{self, Write};
use std::time::Instant;
use colored::*;

mod commands;

#[derive(Deserialize, Debug)]
struct Command {
    command: String,
    data: Value,
}

fn main() {
    println!("{}", "✨ Hey there! Welcome to PebbleVault 👋".green().bold());
    PebbleVault::main();
    PebbleVault::create_db();
    loop {
        print!("{}", "> ".yellow().bold());
        io::stdout().flush().unwrap();

        // Read the command from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Remove the trailing newline character
        let input = input.trim();

        // Exit the loop if the user types "exit"
        if input == "exit" {
            break;
        }

        // Parse the input as a JSON string
        match serde_json::from_str::<Command>(input) {
            Ok(command) => {
                // println!("{}", format!("🔍 Received command: {:?}", command).blue().bold());

                // Start timing the command execution
                let start = Instant::now();

                // Handle the command here
                match command.command.as_str() {
                    "print" => commands::print::execute(&command.data),
                    "add" => commands::add::execute(&command.data),
                    "create_db" => commands::create_db::execute(),
                    _ => println!("{}", "❓ Unknown command".red().bold()),
                }

                // Calculate and print the duration
                let duration = start.elapsed();
                println!("{}", format!("⏱️ Executed in: {:?}", duration).magenta().bold());
            }
            Err(e) => println!("{}", format!("❌ Failed to parse command: {}", e).red().bold()),
        }
    }
}
