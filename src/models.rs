use std::error::Error;

use serde::{Deserialize, Serialize};

// Make the struct and its fields 'pub' so other modules can access them
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub id: String,
    pub timestamp: String,
    pub value: f64,
    pub tag: Option<String>,
}

// (Optional) You can add logic specific to the Record here later
impl Record {
    pub fn new(id: String, value: f64, tag: String) -> Self {
        Self {
            id,
            timestamp: chrono::Utc::now().to_rfc3339(),
            value,
            tag: Some(tag),
        }
    }
}

pub struct HelpInfo {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub description: &'static str,
    pub usage: &'static str,
}

// The contract every command module must sign
pub trait Command {
    // Returns the help info
    fn info(&self) -> HelpInfo;

    // Executes the logic (taking the args list, not the whole string)
    fn execute(&self, args: &[&str]) -> Result<(), Box<dyn Error>>;
}
