use axum::Json;

use crate::{db::ProfileDb, models::RegisterUserPayload};
use common::error::Result;

// [TODO]
// POST /v1/signup
pub async fn create_user_profile(Json(payload): Json<RegisterUserPayload>) -> Result<()> {
    let db = ProfileDb::new().await?;
    db.create_user(payload).await?;

    // TODO:
    //      - Store registeration payload into Database

    Ok(())
}
