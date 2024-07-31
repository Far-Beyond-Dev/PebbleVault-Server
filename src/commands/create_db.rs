use PebbleVault;
use ffi;

pub fn execute() {
  let db = ffi::CreateDB();
  let db = PebbleVault::create_db();
  println!("DB created: {:?}", db);
}
