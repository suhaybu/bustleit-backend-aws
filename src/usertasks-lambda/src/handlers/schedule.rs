use axum::{
    extract::{Path, Query},
    Json,
};
use chrono::NaiveDate;

use crate::db::UserTasksDb;
use crate::models::{
    query::{DateRangeQuery, DATE_FMT},
    response::ScheduleResponse,
};
use common::error::{Error, Result};

/// Retrieves a user's schedule for a specified time period
///
/// # Endpoint
/// ```text
/// GET /v1/user/:user_id/schedule
/// ```
///
/// # Query Parameters
/// - `date`: Optional starting date in YYYY-MM-DD format. Defaults to today if not provided
/// - `until`: Optional end date in YYYY-MM-DD format
/// - `range`: Optional number of days to fetch (1-31). Cannot be used with 'until'
/// - `skip_empty`: Optional bool, if true, does not return any empty schedules
///
/// # Examples
///
/// Single day schedule:
/// ```text
/// GET /v1/user/542172eb-c417-46c0-b9b1-78d1b7630bf5/schedule?date=2024-11-07
/// ```
///
/// Date range with end date:
/// ```text
/// GET /v1/user/542172eb-c417-46c0-b9b1-78d1b7630bf5/schedule?date=2024-11-07&until=2024-11-14
/// ```
///
/// Date range with number of days and skip empty:
/// ```text
/// GET /v1/user/542172eb-c417-46c0-b9b1-78d1b7630bf5/schedule?date=2024-11-07&range=7&skip_empty=true
/// ```
///
/// Today's schedule (default):
/// ```text
/// GET /v1/user/542172eb-c417-46c0-b9b1-78d1b7630bf5/schedule
/// ```
///
/// # Returns
/// - `200 OK`: Schedule data for the requested time period
/// - `400 Bad Request`: Invalid date format or range parameters
/// - `500 Internal Server Error`: Database or server errors
///
/// # Flow
/// 1. Validates query parameters for date formats and logical consistency
/// 2. Establishes database connection
/// 3. Determines date range based on query parameters
/// 4. Retrieves tasks from database
/// 5. Builds response including empty days within the range
pub async fn get_user_schedule(
    Path(user_id): Path<String>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<Option<ScheduleResponse>>> {
    query.validate_all()?;
    let db = UserTasksDb::new().await?;

    // If no start date is supplied, will take today's date
    let start_date = query
        .date
        .unwrap_or_else(|| chrono::Local::now().format(DATE_FMT).to_string());

    let (end_date, is_range_query) = if let Some(until) = query.until {
        (until, true)
    } else if let Some(range) = query.range {
        let end = NaiveDate::parse_from_str(&start_date, DATE_FMT)
            .unwrap() // Safe due to validation
            .checked_add_days(chrono::Days::new((range - 1) as u64))
            .ok_or_else(|| Error::validation("Invalid date range calculation"))?
            .format(DATE_FMT)
            .to_string();
        (end, true)
    } else {
        (start_date.clone(), false)
    };

    // Fetch tasks from DB
    let tasks = db
        .get_user_schedule(&user_id, &start_date, Some(&end_date))
        .await?;

    if !is_range_query && query.skip_empty && tasks.is_empty() {
        return Ok(Json(None));
    }

    let mut response = ScheduleResponse::new(user_id);

    for task in tasks {
        if !task.tasks.is_empty() || !query.skip_empty {
            response.add_day(task.date.clone(), Some(task));
        }
    }

    // Handles calculating the end date if query was given a range
    if is_range_query {
        let start = NaiveDate::parse_from_str(&start_date, DATE_FMT).unwrap();
        let end = NaiveDate::parse_from_str(&end_date, DATE_FMT).unwrap();
        let mut current = start;

        while current <= end {
            let current_date = current.format(DATE_FMT).to_string();
            if !response.data.contains_key(&current_date) && !query.skip_empty {
                response.add_day(current_date, None);
            }
            current = current.succ_opt().unwrap();
        }
    }

    if query.skip_empty && response.data.is_empty() {
        return Ok(Json(None));
    }

    Ok(Json(Some(response)))
}