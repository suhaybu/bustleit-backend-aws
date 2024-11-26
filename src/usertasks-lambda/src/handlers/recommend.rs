use axum::extract::Path;
use common::error::Result;
use uuid::Uuid;

// GET: /v1/recommend/:user_id
pub async fn get_recommendation(Path(user_id): Path<Uuid>) -> Result<()> {
    todo!()
}
