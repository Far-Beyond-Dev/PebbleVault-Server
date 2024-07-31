use serde_json::Value;

pub fn execute(data: &Value) {
    if let Some(text) = data["text"].as_str() {
        println!("{}", text);
    } else {
        println!("Invalid data for print command");
    }
}
