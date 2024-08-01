use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let limit = args.get(1).and_then(|s| s.parse::<usize>().ok());

    if let Ok(lines) = read_lines("history.txt") {
        let mut count = 0;
        for line in lines {
            if let Ok(content) = line {
                println!("{}", content);
                count += 1;
                if let Some(limit) = limit {
                    if count >= limit {
                        break;
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to read history file");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}