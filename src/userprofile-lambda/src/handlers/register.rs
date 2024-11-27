use axum::Json;

use crate::{db::ProfileDb, models::RegisterUserPayload};
use common::error::Result;

// POST /v1/signup
pub async fn create_user_profile(Json(payload): Json<RegisterUserPayload>) -> Result<()> {
    let db = ProfileDb::new().await?;
    let response = db.create_user(payload).await?;

    Ok(response)
}
