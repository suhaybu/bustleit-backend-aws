use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateClustersPayload(pub Vec<Cluster>);

#[derive(Deserialize)]
pub struct Cluster {
    pub cluster: ClusterData,
}

#[derive(Deserialize)]
pub struct ClusterData {
    pub number: i32,
    pub users: Vec<Uuid>,
}
