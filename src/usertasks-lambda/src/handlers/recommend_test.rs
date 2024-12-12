use std::time::Duration;

use axum::{
    extract::{Path, Query},
    Json,
};
use chrono::{Datelike, NaiveDate, Timelike};
use rand::seq::SliceRandom;
use serde::Deserialize;
use tokio::time::sleep;
use uuid::Uuid;

use crate::data::ALL_SCHEDULES;
use crate::models::response::{DayTasks, ScheduleResponse, Task};
use common::error::{Error, Result};

#[derive(Deserialize)]
pub struct DateQuery {
    date: Option<String>,
}

pub async fn get_recommendation(
    Path(_user_id): Path<Uuid>,
    Query(query): Query<DateQuery>,
) -> Result<Json<ScheduleResponse>> {
    let date = if let Some(date_str) = query.date {
        NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|_| Error::validation("Invalid date format, expected YYYY-MM-DD"))?
    } else {
        chrono::Local::now().date_naive()
    };

    let day_name = match date.weekday() {
        chrono::Weekday::Mon => "Monday",
        chrono::Weekday::Tue => "Tuesday",
        chrono::Weekday::Wed => "Wednesday",
        chrono::Weekday::Thu => "Thursday",
        chrono::Weekday::Fri => "Friday",
        chrono::Weekday::Sat => "Saturday",
        chrono::Weekday::Sun => "Sunday",
    };

    // uses random generator
    // let schedule = ALL_SCHEDULES
    //     .choose(&mut rand::thread_rng())
    //     .ok_or_else(|| Error::InternalServerError("Failed to select recommendation".into()))?;

    // Every 5 seconds, switches to a different response to randomize
    let seconds = chrono::Utc::now().second() as usize;
    let index = (seconds / 5) % 5; // This will give 0-11 sequence repeating
    let schedule = &ALL_SCHEDULES[index];

    if let Some(day_schedule) = schedule.get(day_name) {
        let tasks = day_schedule["tasks"]
            .as_array()
            .ok_or_else(|| Error::InternalServerError("Invalid tasks array".into()))?
            .iter()
            .map(|t| {
                let formatted_date = date.format("%Y-%m-%d").to_string();
                let start_datetime = format!(
                    "{} {} UTC",
                    formatted_date,
                    t["startTime"].as_str().unwrap_or_default()
                );
                let end_datetime = format!(
                    "{} {} UTC",
                    formatted_date,
                    t["endTime"].as_str().unwrap_or_default()
                );

                Task {
                    task_id: Uuid::new_v4().to_string(),
                    name: t["name"].as_str().unwrap_or_default().to_string(),
                    category: "Activity".to_string(),
                    start_time: start_datetime,
                    end_time: end_datetime,
                    completed: false,
                    created_at: chrono::Utc::now().to_string(),
                    updated_at: chrono::Utc::now().to_string(),
                }
            })
            .collect();

        let mut response = ScheduleResponse::new(_user_id.to_string());
        response.data.insert(
            date.format("%Y-%m-%d").to_string(),
            DayTasks {
                total_tasks: day_schedule["tasks"].as_array().unwrap().len() as i32,
                completed_tasks: 0,
                tasks,
            },
        );

        sleep(Duration::from_millis(1500)).await;

        Ok(Json(response))
    } else {
        Err(Error::not_found(format!(
            "No schedule found for {}",
            day_name
        )))
    }
}

pub async fn get_recommendation_week(Path(_user_id): Path<Uuid>) -> Result<Json<ScheduleResponse>> {
    let schedule = ALL_SCHEDULES
        .choose(&mut rand::thread_rng())
        .ok_or_else(|| Error::InternalServerError("Failed to select recommendation".into()))?;

    let mut response = ScheduleResponse::new(_user_id.to_string());

    for (_day, day_schedule) in schedule.as_object().unwrap() {
        let tasks = day_schedule["tasks"]
            .as_array()
            .ok_or_else(|| Error::InternalServerError("Invalid tasks array".into()))?
            .iter()
            .map(|t| Task {
                task_id: Uuid::new_v4().to_string(),
                name: t["name"].as_str().unwrap_or_default().to_string(),
                category: "Activity".to_string(),
                start_time: t["startTime"].as_str().unwrap_or_default().to_string(),
                end_time: t["endTime"].as_str().unwrap_or_default().to_string(),
                completed: false,
                created_at: chrono::Utc::now().to_string(),
                updated_at: chrono::Utc::now().to_string(),
            })
            .collect();

        response.data.insert(
            day_schedule["date"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            DayTasks {
                total_tasks: day_schedule["tasks"].as_array().unwrap().len() as i32,
                completed_tasks: 0,
                tasks,
            },
        );
    }

    Ok(Json(response))
}
