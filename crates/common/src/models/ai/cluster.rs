use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ClusteredUsers {
    pub id: i32,
    pub cluster: i32,
}
