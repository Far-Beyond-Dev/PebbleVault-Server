use PebbleVault;  // Note: use lowercase 'pebble_vault'

pub fn main() {
  let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
      PebbleVault::close_db();
        println!("Hello, {}!", args[1]);
    } else {
        println!("Hello, World!");
    }
}


