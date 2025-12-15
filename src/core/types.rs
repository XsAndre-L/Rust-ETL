use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub id: String,
    pub timestamp: String,
    pub value: f64,
    pub tag: Option<String>,
}

pub struct HelpInfo {
    pub label: &'static str,
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
