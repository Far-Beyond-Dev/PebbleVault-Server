use PebbleVault;  // Note: use lowercase 'pebble_vault'

pub fn main() {
  let db = PebbleVault::create_db();
  println!("DB created: {:?}", db);
}
