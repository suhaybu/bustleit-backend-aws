use axum::{extract::Path, extract::Query, Json};
use chrono::NaiveDate;
use std::collections::BTreeMap;
use uuid::Uuid;

use common::error::{Error, Result};

use crate::db::TasksDb;
use crate::models::query::{DateRangeQuery, DATE_FMT};
use crate::models::response::{DayTasks, ScheduleResponse, Task};

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
pub async fn get_user_schedule(
    Path(user_id): Path<Uuid>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<Option<ScheduleResponse>>> {
    query.validate_all()?;

    let db = TasksDb::new().await?;

    // Get start date (or today if not provided)
    let start_date = query
        .date
        .as_ref()
        .map(|d| NaiveDate::parse_from_str(d, DATE_FMT))
        .transpose()
        .map_err(|_| Error::validation("Invalid start date format"))?
        .unwrap_or_else(|| chrono::Local::now().date_naive());

    // Calculate end date if range or until is provided
    let end_date = if let Some(until) = &query.until {
        Some(
            NaiveDate::parse_from_str(until, DATE_FMT)
                .map_err(|_| Error::validation("Invalid end date format"))?,
        )
    } else if let Some(range) = query.range {
        Some(
            start_date
                .checked_add_days(chrono::Days::new((range - 1) as u64))
                .ok_or_else(|| Error::validation("Invalid date range calculation"))?,
        )
    } else {
        None
    };

    // Fetch schedule and tasks from database
    let (schedules, tasks) = db.get_user_schedule(user_id, start_date, end_date).await?;

    if schedules.is_empty() && query.skip_empty {
        return Ok(Json(None));
    }

    let mut response = ScheduleResponse::new(user_id.to_string());

    // Group tasks by date
    let mut task_groups = BTreeMap::new();
    for task in tasks {
        task_groups
            .entry(task.schedule_date)
            .or_insert_with(Vec::new)
            .push(task);
    }

    // Process each schedule
    for schedule in schedules {
        let date_str = schedule.schedule_date.format(DATE_FMT).to_string();
        let tasks = task_groups
            .remove(&schedule.schedule_date)
            .unwrap_or_default();

        if !tasks.is_empty() || !query.skip_empty {
            response.data.insert(
                date_str,
                DayTasks {
                    total_tasks: schedule.total_tasks,
                    completed_tasks: schedule.completed_tasks,
                    tasks: tasks.into_iter().map(Task::from).collect(),
                },
            );
        }
    }

    // Fill in empty days for date ranges
    if let Some(end_date) = end_date {
        let mut current = start_date;
        while current <= end_date {
            let date_str = current.format(DATE_FMT).to_string();
            if !response.data.contains_key(&date_str) && !query.skip_empty {
                response.data.insert(
                    date_str,
                    DayTasks {
                        total_tasks: 0,
                        completed_tasks: 0,
                        tasks: Vec::new(),
                    },
                );
            }
            current = current
                .checked_add_days(chrono::Days::new(1))
                .expect("Invalid date calculation");
        }
    }

    if query.skip_empty && response.data.is_empty() {
        return Ok(Json(None));
    }

    Ok(Json(Some(response)))
}
