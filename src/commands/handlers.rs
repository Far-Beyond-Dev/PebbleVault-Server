use serde_json::Value;

pub trait CommandHandler {
    fn execute(&self, data: &Value);
}
