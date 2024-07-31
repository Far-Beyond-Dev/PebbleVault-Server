use PebbleVault;

pub fn main() {
  let args: Vec<String> = std::env::args().collect();
  
  if args.len() >= 4 {
      let db = args[1].parse::<usize>().unwrap();
      let key = &args[2];
      let value = args[3..].join(" "); // Join all remaining arguments as the value
      
      println!("Setting object in database {} with key '{}' and value '{}'", db, key, value);
      
      PebbleVault::set_object(db, key, &value);
      
      println!("Object set in database {} with key '{}' and value '{}'", db, key, value);
  } else {
      println!("Usage: program <database_pointer> <key> <value>");
  }
}