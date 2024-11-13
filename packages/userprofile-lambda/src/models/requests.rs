use serde::Deserialize;

// POST Request structure for /user/profiles/batch
#[derive(Deserialize)]
pub struct UserProfilesBatchRequest {
    pub user_ids: Vec<String>,
}
