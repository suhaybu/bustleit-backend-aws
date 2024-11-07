use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStats {
    #[serde(rename = "PK")]
    pub pk: String, // STATS#USER#uuid
    #[serde(rename = "SK")]
    pub sk: String, // CATEGORY#type#YEAR#yyyy#MONTH#mm
    pub total_tasks: i32,
    pub completed_tasks: i32,
    pub total_duration: i32,
    pub categories: HashMap<String, i32>,
}

impl UserStats {
    pub fn new(uuid: &str, category_type: &str, year: i32, month: i32) -> Self {
        Self {
            pk: format!("STATS#USER#{}", uuid),
            sk: format!(
                "CATEGORY#{}#YEAR#{}#MONTH#{:02}",
                category_type, year, month
            ),
            total_tasks: 0,
            completed_tasks: 0,
            total_duration: 0,
            categories: HashMap::new(),
        }
    }
}
