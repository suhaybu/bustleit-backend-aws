use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use sqlx::{postgres::PgRow, PgPool, Row};
use uuid::Uuid;

use common::{
    database::DatabaseConfig,
    error::{Error, Result},
    models::database as DB,
};

pub struct TasksDb {
    pool: PgPool,
}

impl TasksDb {
    pub async fn new() -> Result<Self> {
        let config = DatabaseConfig::new()?;
        let pool = common::database::create_pool(config).await?;

        Ok(Self { pool })
    }

    /// Gets all tasks of all users
    pub async fn get_all_tasks(&self) -> Result<Vec<DB::Task>> {
        let rows = sqlx::query(
            "SELECT id, user_id, schedule_date, name, category,
                    start_time, end_time, completed, created_at, updated_at
             FROM tasks
             ORDER BY user_id, schedule_date, start_time",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from)?;

        let tasks = rows
            .into_iter()
            .map(Self::map_task_row)
            .collect::<Result<Vec<_>>>()?;

        Ok(tasks)
    }

    /// Get all tasks for batch of users
    pub async fn get_users_tasks(&self, user_ids: &[Uuid]) -> Result<Vec<DB::Task>> {
        let rows = sqlx::query(
            "SELECT id, user_id, schedule_date, name, category,
                    start_time, end_time, completed, created_at, updated_at
             FROM tasks
             WHERE user_id = ANY($1)
             ORDER BY user_id, schedule_date, start_time",
        )
        .bind(user_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from)?;

        let tasks = rows
            .into_iter()
            .map(Self::map_task_row)
            .collect::<Result<Vec<_>>>()?;

        Ok(tasks)
    }

    /// Get a user's schedule
    pub async fn get_user_schedule(
        &self,
        user_id: Uuid,
        date: NaiveDate,
        date_end: Option<NaiveDate>,
    ) -> Result<(Vec<DB::Schedule>, Vec<DB::Task>)> {
        match date_end {
            Some(end_date) => self.get_user_schedule_range(user_id, date, end_date).await,
            None => {
                let (schedule, tasks) = self.get_user_schedule_single_day(user_id, date).await?;
                Ok((vec![schedule], tasks))
            }
        }
    }

    /// Add task to a user
    pub async fn add_task(
        &self,
        user_id: Uuid,
        date: NaiveDate,
        name: &str,
        category: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<DB::Task> {
        let mut tx = self.pool.begin().await.map_err(Error::from)?;

        let start_time = Self::format_time(date, start_time)?;
        let end_time = Self::format_time(date, end_time)?;

        // Insert or update schedule
        sqlx::query(
            "INSERT INTO schedules (user_id, schedule_date, total_tasks, completed_tasks)
             VALUES ($1, $2, 1, 0)
             ON CONFLICT (user_id, schedule_date)
             DO UPDATE SET total_tasks = schedules.total_tasks + 1",
        )
        .bind(user_id)
        .bind(date)
        .execute(&mut *tx)
        .await
        .map_err(Error::from)?;

        // Insert task
        let task = sqlx::query(
            "INSERT INTO tasks
             (user_id, schedule_date, name, category, start_time, end_time)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, user_id, schedule_date, name, category,
                       start_time, end_time, completed, created_at, updated_at",
        )
        .bind(user_id)
        .bind(date)
        .bind(name)
        .bind(category)
        .bind(start_time)
        .bind(end_time)
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::from)
        .map(Self::map_task_row)??;

        tx.commit().await.map_err(Error::from)?;

        Ok(task)
    }

    /// Update a user's task
    pub async fn update_task(
        &self,
        user_id: Uuid,
        task_id: Uuid,
        name: Option<&str>,
        category: Option<&str>,
        start_time: Option<&str>,
        end_time: Option<&str>,
        completed: Option<bool>,
        date: Option<NaiveDate>,
    ) -> Result<DB::Task> {
        let mut tx = self.pool.begin().await.map_err(Error::from)?;

        // Get task current state
        let current_task = sqlx::query(
            "SELECT schedule_date, completed, start_time, end_time
             FROM tasks
             WHERE id = $1 AND user_id = $2",
        )
        .bind(task_id)
        .bind(user_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(Error::from)?
        .ok_or_else(|| Error::not_found(format!("Task {}", task_id)))?;

        let current_date: NaiveDate = current_task.get("schedule_date");
        let current_completed: bool = current_task.get("completed");

        // Now we handle each Optional data type

        if let Some(new_data) = date {
            let completed_int: i32 = if current_completed { 1 } else { 0 };
            // If new date is different than old date, we update new & old schedules
            if new_data != current_date {
                // Update current (old) schedule counter
                sqlx::query(
                    "UPDATE schedules
                     SET total_tasks = total_tasks - 1,
                        completed_tasks = completed_tasks - $3
                     WHERE user_id = $1 AND schedule_date = $2",
                )
                .bind(user_id) // $1
                .bind(current_date) // $2
                .bind(completed_int) // $3
                .execute(&mut *tx)
                .await
                .map_err(Error::from)?;

                // Updates new schedule
                sqlx::query(
                    "INSERT INTO schedules (user_id, schedule_date, total_tasks, completed_tasks)
                     VALUES ($1, $2, 1, $3)
                     ON CONFLICT (user_id, schedule_date)
                     DO UPDATE SET
                        total_tasks = schedules.total_tasks + 1,
                        completed_tasks = schedules.completed_tasks + $3",
                )
                .bind(user_id)
                .bind(new_data)
                .bind(completed_int)
                .execute(&mut *tx)
                .await
                .map_err(Error::from)?;
            }
        }

        if let Some(new_completed) = completed {
            if new_completed != current_completed {
                let schedule_date = date.unwrap_or(current_date); // Use new date if provided
                let completed_change: i32 = if new_completed { 1 } else { -1 };

                sqlx::query(
                    "UPDATE schedules
                     SET completed_tasks = completed_tasks + $3
                     WHERE user_id = $1 AND schedule_date = $2",
                )
                .bind(user_id)
                .bind(schedule_date)
                .bind(completed_change)
                .execute(&mut *tx)
                .await
                .map_err(Error::from)?;
            }
        }

        // If start time is specified, use that
        let start_time = if let Some(time) = start_time {
            Some(Self::format_time(date.unwrap_or(current_date), time)?)
        } else {
            None
        };

        // If end time is specified, use that
        let end_time = if let Some(time) = end_time {
            Some(Self::format_time(date.unwrap_or(current_date), time)?)
        } else {
            None
        };

        // Validated that end time is later than start time
        if let (Some(start), Some(end)) = (start_time.as_ref(), end_time.as_ref()) {
            if end <= start {
                return Err(Error::validation("End time must be after start time"));
            }
        }

        // Build and execute the update query
        let updated = sqlx::query_as::<_, DB::Task>(
            "UPDATE tasks SET
                name = COALESCE($1, name),
                category = COALESCE($2, category),
                start_time = COALESCE($3, start_time),
                end_time = COALESCE($4, end_time),
                completed = COALESCE($5, completed),
                schedule_date = COALESCE($6, schedule_date),
                updated_at = CURRENT_TIMESTAMP
             WHERE id = $7 AND user_id = $8
             RETURNING *",
        )
        .bind(name)
        .bind(category)
        .bind(start_time)
        .bind(end_time)
        .bind(completed)
        .bind(date)
        .bind(task_id)
        .bind(user_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::from)?;

        tx.commit().await.map_err(Error::from)?;

        Ok(updated)
    }

    /// Delete a user's task
    pub async fn delete_task(&self, user_id: Uuid, task_id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await.map_err(Error::from)?;

        // Get task info
        let task_row = sqlx::query(
            "SELECT schedule_date, completed FROM tasks WHERE id = $1 AND user_id = $2",
        )
        .bind(task_id)
        .bind(user_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(Error::from)?
        .ok_or_else(|| Error::not_found(format!("Task {}", task_id)))?;

        // Delete task
        sqlx::query("DELETE FROM tasks WHERE id = $1")
            .bind(task_id)
            .execute(&mut *tx)
            .await
            .map_err(Error::from)?;

        // Update schedule
        sqlx::query(
            "UPDATE schedules
                 SET total_tasks = total_tasks - 1,
                     completed_tasks = completed_tasks - CASE WHEN $3 THEN 1 ELSE 0 END
                 WHERE user_id = $1 AND schedule_date = $2",
        )
        .bind(user_id)
        .bind(task_row.get::<NaiveDate, _>("schedule_date"))
        .bind(task_row.get::<bool, _>("completed"))
        .execute(&mut *tx)
        .await
        .map_err(Error::from)?;

        tx.commit().await.map_err(Error::from)?;

        Ok(())
    }

    /// Helper function to get a schedule for a single day
    async fn get_user_schedule_single_day(
        &self,
        user_id: Uuid,
        date: NaiveDate,
    ) -> Result<(DB::Schedule, Vec<DB::Task>)> {
        // Get or create schedule
        let schedule_row = sqlx::query(
            "SELECT user_id, schedule_date, completed_tasks, total_tasks, created_at, updated_at
             FROM schedules
             WHERE user_id = $1 AND schedule_date = $2",
        )
        .bind(user_id)
        .bind(date)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from)?;

        let schedule = match schedule_row {
            Some(row) => DB::Schedule {
                user_id: row.get("user_id"),
                schedule_date: row.get("schedule_date"),
                completed_tasks: row.get("completed_tasks"),
                total_tasks: row.get("total_tasks"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            },
            None => DB::Schedule {
                user_id,
                schedule_date: date,
                completed_tasks: 0,
                total_tasks: 0,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        };

        // Get tasks
        let task_rows = sqlx::query(
            "SELECT id, user_id, schedule_date, name, category,
                    start_time, end_time, completed, created_at, updated_at
             FROM tasks
             WHERE user_id = $1 AND schedule_date = $2
             ORDER BY start_time",
        )
        .bind(user_id)
        .bind(date)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from)?;

        let tasks = task_rows
            .into_iter()
            .map(Self::map_task_row)
            .collect::<Result<Vec<_>>>()?;

        Ok((schedule, tasks))
    }

    /// Helper function get a schedule for a range of days
    async fn get_user_schedule_range(
        &self,
        user_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<(Vec<DB::Schedule>, Vec<DB::Task>)> {
        // Get schedules
        let schedule_rows = sqlx::query(
            "SELECT user_id, schedule_date, completed_tasks, total_tasks, created_at, updated_at
             FROM schedules
             WHERE user_id = $1 AND schedule_date BETWEEN $2 AND $3
             ORDER BY schedule_date",
        )
        .bind(user_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from)?;

        let schedules = schedule_rows
            .into_iter()
            .map(|row| DB::Schedule {
                user_id: row.get("user_id"),
                schedule_date: row.get("schedule_date"),
                completed_tasks: row.get("completed_tasks"),
                total_tasks: row.get("total_tasks"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        // Get tasks
        let task_rows = sqlx::query(
            "SELECT id, user_id, schedule_date, name, category,
                    start_time, end_time, completed, created_at, updated_at
             FROM tasks
             WHERE user_id = $1 AND schedule_date BETWEEN $2 AND $3
             ORDER BY schedule_date, start_time",
        )
        .bind(user_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from)?;

        let tasks = task_rows
            .into_iter()
            .map(Self::map_task_row)
            .collect::<Result<Vec<_>>>()?;

        Ok((schedules, tasks))
    }

    /// Helper function to convert Date -> Time (HH:MM 24-hr UTC)
    fn format_time(date: NaiveDate, time: &str) -> Result<DateTime<Utc>> {
        let naive_time = NaiveTime::parse_from_str(time, "%H:%M")
            .map_err(|_| Error::validation("Time must be in HH:MM format"))?;

        let naive_datetime = date.and_time(naive_time);

        Ok(DateTime::<Utc>::from_naive_utc_and_offset(
            naive_datetime,
            Utc,
        ))
    }

    fn map_task_row(row: PgRow) -> Result<DB::Task> {
        Ok(DB::Task {
            id: row.get("id"),
            user_id: row.get("user_id"),
            schedule_date: row.get("schedule_date"),
            name: row.get("name"),
            category: row.get("category"),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
            completed: row.get("completed"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}
