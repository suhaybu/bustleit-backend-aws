use serde::Deserialize;

#[derive(Deserialize)]
pub struct TasksRequest {
    pub user_ids: Vec<String>,
}
