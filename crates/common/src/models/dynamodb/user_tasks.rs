use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTasks {
    #[serde(rename = "PK")]
    pub pk: String, // USER#uuid
    #[serde(rename = "SK")]
    pub sk: String, // TASK#DATE#YYYY-MM-DD
    #[serde(rename = "GSI1PK")]
    pub gsi1pk: String, // DATE#YYYY-MM-DD
    #[serde(rename = "GSI1SK")]
    pub gsi1sk: String, // USER#uuid
    pub date: String,
    #[serde(rename = "completedTasks")]
    pub completed_tasks: i32,
    #[serde(rename = "totalTasks")]
    pub total_tasks: i32,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub category: String,
    pub completed: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    pub name: String,
    #[serde(rename = "taskId")]
    pub task_id: String,
}

impl UserTasks {
    pub fn new(uuid: &str, date: &str) -> Self {
        Self {
            pk: format!("USER#{}", uuid),
            sk: format!("TASK#DATE#{}", date),
            gsi1pk: format!("DATE#{}", date),
            gsi1sk: format!("USER#{}", uuid),
            date: date.to_string(),
            completed_tasks: 0,
            total_tasks: 0,
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.total_tasks += 1;
        if task.completed {
            self.completed_tasks += 1;
        }
        self.tasks.push(task);
    }
}

impl Task {
    pub fn new(
        name: String,
        category: String,
        start_time: String,
        end_time: String,
        task_id: String,
    ) -> Self {
        let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        Self {
            category,
            completed: false,
            created_at: timestamp.clone(),
            updated_at: timestamp,
            start_time,
            end_time,
            name,
            task_id,
        }
    }

    pub fn parse_timestamp(
        timestamp: &str,
    ) -> Result<chrono::DateTime<chrono::Utc>, chrono::ParseError> {
        chrono::DateTime::parse_from_str(timestamp, "%Y-%m-%dT%H:%M:%SZ")
            .map(|dt| dt.with_timezone(&chrono::Utc))
    }
}
