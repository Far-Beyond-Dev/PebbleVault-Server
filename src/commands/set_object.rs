/*use PebbleVault;

pub fn main() {
  // let args: Vec<String> = std::env::args().collect();
  // let db = args[1].parse::<usize>().unwrap();
  let db = PebbleVault::create_db();
  let key1 = "key1";
  let key2 = "key2";

  PebbleVault::set_object(db, key1, r#"{"type": "car", "uuid": "abc-123", "x": 1.0, "y": 2.0, "z": 3.0}"#);
  PebbleVault::set_object(db, key1, r#"{"type": "car", "uuid": "abc-123", "x": 10.0, "y": 20.0, "z": 30.0}"#);
  PebbleVault::set_object(db, key2, r#"{"type": "car", "uuid": "abc-123", "x": -1.0, "y": -2.0, "z": -3.0}"#);
  println!("{}", PebbleVault::get_object(db, key1));

  /* 
  if args.len() >= 1 {
      
      //let key = &args[2];
      //let value = args[3..].join(" "); // Join all remaining arguments as the value
      
      
      
      // println!("Object set in database {} with key '{}' and value '{}'", db, "key1", r#"{"type": "car", "uuid": "abc-123", "x": 1.0, "y": 2.0, "z": 3.0}"#);
  } else {
      println!("Usage: program <database_pointer> <key> <value>");
  }
  */
}  */