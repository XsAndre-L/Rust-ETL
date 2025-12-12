use serde::{Deserialize, Serialize};

// Make the struct and its fields 'pub' so other modules can access them
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub id: String,
    pub timestamp: String,
    pub value: f64,
    pub tag: String,
}

// (Optional) You can add logic specific to the Record here later
impl Record {
    pub fn new(id: String, value: f64, tag: String) -> Self {
        Self {
            id,
            timestamp: chrono::Utc::now().to_rfc3339(),
            value,
            tag,
        }
    }
}
