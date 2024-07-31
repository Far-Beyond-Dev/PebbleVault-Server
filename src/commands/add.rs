use serde_json::Value;

pub fn execute(data: &Value) {
    if let Some(a) = data["a"].as_i64() {
        if let Some(b) = data["b"].as_i64() {
            println!("Result: {}", a + b);
        } else {
            println!("Invalid data for add command");
        }
    } else {
        println!("Invalid data for add command");
    }
}
