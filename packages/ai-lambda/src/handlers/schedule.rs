// In src/handlers/schedule.rs

use axum::{extract::Path, http::StatusCode, Json};
use chrono::{Datelike, NaiveDate};
use tracing::info;

use common::{dynamodb::DynamoDbClient, models::webapp::user_tasks::FrontendScheduleResponse};

// GET /v1/users/{userId}/schedule/month/{month}
pub async fn get_user_schedule_month(
    Path((user_id, month)): Path<(String, String)>,
) -> Result<Json<FrontendScheduleResponse>, (StatusCode, Json<serde_json::Value>)> {
    info!(
        "Fetching schedule for user: {} for month: {}",
        user_id, month
    );

    let db = DynamoDbClient::new().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;

    // Parse the month (assumed format: YYYY-MM)
    let date = NaiveDate::parse_from_str(&format!("{}-01", month), "%Y-%m-%d").map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("Invalid month format: {}", e) })),
        )
    })?;

    // Calculate the number of days in the month
    let days_in_month = if date.month() == 12 {
        NaiveDate::from_ymd_opt(date.year() + 1, 1, 1)
            .unwrap()
            .signed_duration_since(date)
            .num_days()
    } else {
        NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1)
            .unwrap()
            .signed_duration_since(date)
            .num_days()
    };

    let response = FrontendScheduleResponse {
        user_id: user_id.clone(),
        data: Default::default(),
    };

    // Fetch tasks for each day in the month
    for day in 1..=days_in_month {
        let date = NaiveDate::from_ymd_opt(date.year(), date.month(), day as u32)
            .unwrap()
            .format("%Y-%m-%d")
            .to_string();

        let _ = db
            .get_user_tasks(&user_id, &date, None)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": e.to_string() })),
                )
            })?;

        // if !tasks.tasks.is_empty() {
        //     response
        //         .data
        //         .insert(date, tasks.tasks.iter().map(FrontendTask::from).collect());
        // }
    }

    Ok(Json(response))
}
