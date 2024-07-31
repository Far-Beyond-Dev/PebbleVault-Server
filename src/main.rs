use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Editor};
use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::path::Path;
use std::time::Instant;
use colored::*;
use chrono::Local;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        execute_command(&args[1..]);
    } else {
        run_interactive_cli();
    }
}

fn run_interactive_cli() {
    let ascii_art = r#"
          /\
         /**\
        /****\   /\
       /      \ /**\
      /  /\    /    \        /\    /\  /\      /\            /\/\/\  /\
     /  /  \  /      \      /  \/\/  \/  \  /\/  \/\  /\  /\/ / /  \/  \
    /  /    \/ /\     \    /    \ \  /    \/ /   /  \/  \/  \  /    \   \
   /  /      \/  \/\   \  /      \    /   /    \
__/__/_______/___/__\___\__________________________________________________        
"#;

println!("{}", ascii_art);
println!("{}", "                          Welcome to PebbleVault CLI 🗿".green().bold());
println!("{}", "                                  Ver: 0.1.0-A".blue().bold());


    let mut rl = DefaultEditor::new().unwrap();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let mut commands: HashMap<String, Box<dyn Fn(&[String])>> = HashMap::new();
    commands.insert("create_db".to_string(), Box::new(execute_command));
    commands.insert("close_db".to_string(), Box::new(execute_command));
    commands.insert("set_object".to_string(), Box::new(execute_command));
    commands.insert("get_object".to_string(), Box::new(execute_command));
    commands.insert("greet".to_string(), Box::new(execute_command));
    commands.insert("use_db".to_string(), Box::new(execute_command));

    println!("Type 'help' for a list of commands or 'exit' to quit.");

    // Get the current Unix user
    let username = env::var("USER").unwrap_or_else(|_| "unknown".to_string());

    // Initialize current database handle (you might want to implement a way to change this)
    let mut current_db = "default".to_string();

    loop {
        let prompt = format!(
            "{} {}@{} [{}] PebbleVault >> ",
            Local::now().format("%H:%M:%S").to_string().blue(),
            username.green(),
            current_db.yellow(),
            env::current_dir().unwrap().display().to_string().cyan()
        );
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                let args: Vec<String> = line.split_whitespace().map(String::from).collect();
                if let Some((cmd, args)) = args.split_first() {
                    match cmd.as_str() {
                        "exit" => {
                            println!("{}", "Goodbye! 👋".yellow());
                            break;
                        },
                        "help" => print_help(&commands),
                        "use_db" => {
                            if let Some(db_name) = args.first() {
                                current_db = db_name.to_string();
                                println!("{} {}", "Switched to database:".green(), current_db);
                            } else {
                                println!("{}", "Please specify a database name.".red());
                            }
                        },
                        _ => {
                            if let Some(func) = commands.get(cmd) {
                                let mut full_args = vec![cmd.to_string()];
                                full_args.extend_from_slice(args);
                                let start = Instant::now();
                                func(&full_args);
                                let duration = start.elapsed();
                                println!("{} {:.2?}", "Command executed in".italic(), duration);
                            } else {
                                println!("{}: {} Type 'help' for a list of commands.", "Unknown command! ❌".red(), cmd.to_string());
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "CTRL-C".red());
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "CTRL-D".red());
                break;
            }
            Err(err) => {
                println!("{}: {:?}", "Error".red().bold(), err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

fn execute_command(args: &[String]) {
    if args.is_empty() {
        println!("{}", "No command specified ❌".red());
        return;
    }

    let command_name = &args[0];
    let current_exe = env::current_exe().unwrap();
    let bin_dir = current_exe.parent().unwrap();
    let command_path = bin_dir.join(command_name);

    if !command_path.exists() {
        println!("{}: {}", "Command not found ❌".red(), command_name);
        return;
    }

    println!("{} {}", "Executing command".cyan(), command_name);
    let status = Command::new(&command_path)
        .args(&args[1..])
        .status()
        .expect("Failed to execute command");

    if status.success() {
        println!("{}", "Command executed successfully ✅".green());
    } else {
        eprintln!("{}: {}", "Command failed ❌".red(), command_name);
    }
}

///////////////////////////////////////////////////////////
//                   Help command UI                     //
//  Everything in this section is related to generating  //
//  the help UI                                          //
///////////////////////////////////////////////////////////

fn print_help(commands: &HashMap<String, Box<dyn Fn(&[String])>>) {
    let box_width = 76; // Width of the box
    let border = "═".repeat(box_width - 2);
    let top_border = format!("╭{}╮", border);
    let bottom_border = format!("╰{}╯", border);

    // Print the top border of the box
    println!("{}", top_border.yellow().bold());

    // Print the title inside the box
    println!("│ {} │", "Available commands:".yellow().underline().pad_to_width_with_alignment(box_width - 5, Alignment::Center));
    
    // Print each command inside the box
    for cmd in commands.keys() {
        println!("│ {} {} │", "•".yellow(), cmd.pad_to_width(box_width - 6));
    }

    // Print the exit message inside the box
    println!("│ {} │", "Type 'exit' to quit the program.".italic().pad_to_width_with_alignment(box_width - 3, Alignment::Center));

    // Print the bottom border of the box
    println!("{}", bottom_border.yellow().bold());
}

// Extension trait for padding strings to a specific width with alignment
trait PadToWidth {
    fn pad_to_width(&self, width: usize) -> String;
    fn pad_to_width_with_alignment(&self, width: usize, alignment: Alignment) -> String;
}

impl PadToWidth for str {
    fn pad_to_width(&self, width: usize) -> String {
        let len = self.chars().count();
        if len >= width {
            self.to_string()
        } else {
            let padding = " ".repeat(width - len);
            format!("{}{}", self, padding)
        }
    }

    fn pad_to_width_with_alignment(&self, width: usize, alignment: Alignment) -> String {
        match alignment {
            Alignment::Center => {
                let len = self.chars().count();
                if len >= width {
                    self.to_string()
                } else {
                    let padding = " ".repeat((width - len) / 2);
                    format!("{}{}{}", padding, self, padding)
                }
            },
            Alignment::Left => self.pad_to_width(width),
            Alignment::Right => {
                let len = self.chars().count();
                if len >= width {
                    self.to_string()
                } else {
                    let padding = " ".repeat(width - len);
                    format!("{}{}", padding, self)
                }
            },
        }
    }
}

enum Alignment {
    Left,
    Center,
    Right,
}