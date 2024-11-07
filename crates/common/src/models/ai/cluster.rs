use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ClusteredUsers {
    pub user_id: String,
    pub cluster: i32,
}
