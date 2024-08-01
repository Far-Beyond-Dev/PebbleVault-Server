/* use PebbleVault;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].parse::<usize>() {
            Ok(db) => {
                PebbleVault::close_db(db);
                println!("Closed database: {}", db);
            }
            Err(_) => {
                println!("Error: Unable to parse '{}' as a usize", args[1]);
            }
        }
    } else {
        println!("Usage: close_db <database_pointer>");
    }
}*/