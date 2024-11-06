use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ClusteredUsers {
    pub id: String,
    pub cluster: i32,
}
