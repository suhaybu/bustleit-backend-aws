use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TasksQuery {
    date: Option<String>, // Format: YYYY-MM-DD
}

#[derive(Serialize)]
pub struct TimeValidationError {
    message: String,
    details: Option<String>,
}
