use aws_sdk_dynamodb::types::AttributeValue;
use common::{
    dynamodb::DynamoDbClient,
    error::{Error, Result},
    models::dynamodb::{Task, UserTasks},
};
use std::collections::HashMap;

pub struct UserTasksDb {
    db: DynamoDbClient,
}

impl UserTasksDb {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            db: DynamoDbClient::new().await?,
        })
    }

    pub async fn get_all_users_tasks(&self) -> Result<Vec<UserTasks>> {
        let mut all_tasks = Vec::new();
        let mut exclusive_start_key: Option<HashMap<String, AttributeValue>> = None;

        loop {
            // tracing::debug!("Scanning for user tasks");

            let mut scan = self
                .db
                .client
                .scan()
                .table_name(&self.db.table_name)
                .filter_expression("begins_with(SK, :task_prefix)")
                .expression_attribute_values(
                    ":task_prefix",
                    AttributeValue::S("TASK#DATE#".to_string()),
                );

            if let Some(last_key) = &exclusive_start_key {
                for (k, v) in last_key.iter() {
                    scan = scan.exclusive_start_key(k.to_string(), v.clone());
                }
            }

            let result = scan
                .send()
                .await
                .map_err(|e| Error::db_query_error(e.to_string()))?;

            if let Some(items) = result.items {
                // tracing::debug!(batch_size = items.len(), "Retrieved batch of tasks");

                let tasks: Vec<Result<UserTasks>> = items
                    .into_iter()
                    .map(|item| self.convert_to_user_tasks(&item))
                    .collect();

                all_tasks.extend(tasks.into_iter().filter_map(|r| r.ok()));
            }

            exclusive_start_key = result.last_evaluated_key;

            if exclusive_start_key.is_none() {
                break;
            }
        }
        // tracing::debug!(total_tasks = all_tasks.len(), "Retrieved all tasks");
        Ok(all_tasks)
    }

    pub async fn get_users_tasks(&self, user_ids: &[String]) -> Result<Vec<UserTasks>> {
        let mut all_tasks = Vec::new();

        for user_id in user_ids {
            let pk = format!("USER#{}", user_id);

            let result = self
                .db
                .client
                .query()
                .table_name(&self.db.table_name)
                .key_condition_expression("PK = :pk AND begins_with(SK, :task_prefix)")
                .expression_attribute_values(":pk", AttributeValue::S(pk))
                .expression_attribute_values(
                    ":task_prefix",
                    AttributeValue::S("TASK#DATE#".to_string()),
                )
                .send()
                .await
                .map_err(|e| Error::db_query_error(e.to_string()))?;

            if let Some(items) = result.items {
                let tasks: Vec<Result<UserTasks>> = items
                    .into_iter()
                    .map(|item| self.convert_to_user_tasks(&item))
                    .collect();

                all_tasks.extend(tasks.into_iter().filter_map(|r| r.ok()));
            }
        }

        Ok(all_tasks)
    }

    pub async fn get_user_schedule(
        &self,
        user_id: &str,
        date: &str,
        date_end: Option<&str>,
    ) -> Result<Vec<UserTasks>> {
        if let Some(date_end) = date_end {
            self.get_user_schedule_range(user_id, date, date_end).await
        } else {
            let tasks = self.get_user_schedule_single_day(user_id, date).await?;
            Ok(vec![tasks])
        }
    }

    async fn get_user_schedule_single_day(&self, user_id: &str, date: &str) -> Result<UserTasks> {
        let pk = format!("USER#{}", user_id);
        let sk = format!("TASK#DATE#{}", date);

        let result = self
            .db
            .client
            .get_item()
            .table_name(&self.db.table_name)
            .key("PK", AttributeValue::S(pk))
            .key("SK", AttributeValue::S(sk))
            .send()
            .await
            .map_err(|e| Error::db_query_error(e.to_string()))?;

        match result.item {
            Some(item) => self.convert_to_user_tasks(&item),
            None => {
                // If no tasks exist for this date, return an empty UserTasks object
                Ok(UserTasks::new(user_id, date))
            }
        }
    }

    async fn get_user_schedule_range(
        &self,
        user_id: &str,
        date_start: &str,
        date_end: &str,
    ) -> Result<Vec<UserTasks>> {
        let pk = DynamoDbClient::create_user_pk(user_id);
        let start_sk = DynamoDbClient::create_task_sk(date_start);
        let end_sk = DynamoDbClient::create_task_sk(date_end);

        let result = self
            .db
            .client
            .query()
            .table_name(&self.db.table_name)
            .key_condition_expression("PK = :pk AND SK BETWEEN :start_sk AND :end_sk")
            .expression_attribute_values(":pk", AttributeValue::S(pk))
            .expression_attribute_values(":start_sk", AttributeValue::S(start_sk))
            .expression_attribute_values(":end_sk", AttributeValue::S(end_sk))
            .send()
            .await
            .map_err(|e| Error::db_query_error(e.to_string()))?;

        match result.items {
            Some(items) => {
                let tasks: Vec<Result<UserTasks>> = items
                    .into_iter()
                    .map(|item| self.convert_to_user_tasks(&item))
                    .collect();

                let valid_tasks: Vec<UserTasks> =
                    tasks.into_iter().filter_map(|result| result.ok()).collect();

                Ok(valid_tasks)
            }

            None => Ok(Vec::new()),
        }
    }

    /// Converts a DynamoDB item into a UserTasks struct
    ///
    /// Takes a HashMap of DynamoDB AttributeValues and converts them into a UserTasks object
    /// by extracting and parsing:
    /// - date from the "date" field
    /// - user ID from the "PK" field (strips "USER#" prefix)
    /// - tasks array from the "tasks" field, where each task contains:
    ///   - name
    ///   - category
    ///   - start time
    ///   - end time
    ///   - task ID
    ///
    /// Returns Result<UserTasks, Error> where:
    /// - Success: Fully populated UserTasks object
    /// - Error: DynamoDbError::ParseError if required fields are missing/malformed
    ///
    /// Used by get_user_tasks() and similar functions to convert raw DynamoDB
    /// responses into the UserTasks domain model.
    fn convert_to_user_tasks(&self, item: &HashMap<String, AttributeValue>) -> Result<UserTasks> {
        let date = item
            .get("date")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| Error::db_parse_error("Missing date"))?
            .clone();

        let uuid = item
            .get("PK")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.trim_start_matches("USER#").to_string())
            .ok_or_else(|| Error::db_parse_error("Missing PK"))?;

        let mut user_tasks = UserTasks::new(&uuid, &date);

        if let Some(AttributeValue::L(tasks)) = item.get("tasks") {
            for task_av in tasks {
                if let AttributeValue::M(task_map) = task_av {
                    let task = Task::new(
                        task_map
                            .get("name")
                            .and_then(|v| v.as_s().ok())
                            .unwrap()
                            .to_string(),
                        task_map
                            .get("category")
                            .and_then(|v| v.as_s().ok())
                            .unwrap()
                            .to_string(),
                        task_map
                            .get("startTime")
                            .and_then(|v| v.as_s().ok())
                            .unwrap()
                            .to_string(),
                        task_map
                            .get("endTime")
                            .and_then(|v| v.as_s().ok())
                            .unwrap()
                            .to_string(),
                        task_map
                            .get("taskId")
                            .and_then(|v| v.as_s().ok())
                            .unwrap()
                            .to_string(),
                    );
                    user_tasks.add_task(task);
                }
            }
        }

        Ok(user_tasks)
    }

    // fn convert_from_user_tasks(
    //     &self,
    //     user_tasks: &UserTasks,
    // ) -> Result<HashMap<String, AttributeValue>> {
    //     let mut item = HashMap::new();

    //     // Add keys
    //     item.insert("PK".to_string(), AttributeValue::S(user_tasks.pk.clone()));
    //     item.insert("SK".to_string(), AttributeValue::S(user_tasks.sk.clone()));
    //     item.insert(
    //         "GSI1PK".to_string(),
    //         AttributeValue::S(user_tasks.gsi1pk.clone()),
    //     );
    //     item.insert(
    //         "GSI1SK".to_string(),
    //         AttributeValue::S(user_tasks.gsi1sk.clone()),
    //     );

    //     // Add metadata
    //     item.insert(
    //         "date".to_string(),
    //         AttributeValue::S(user_tasks.date.clone()),
    //     );
    //     item.insert(
    //         "completedTasks".to_string(),
    //         AttributeValue::N(user_tasks.completed_tasks.to_string()),
    //     );
    //     item.insert(
    //         "totalTasks".to_string(),
    //         AttributeValue::N(user_tasks.total_tasks.to_string()),
    //     );

    //     // Convert tasks to AttributeValue
    //     let tasks: Vec<AttributeValue> = user_tasks
    //         .tasks
    //         .iter()
    //         .map(|task| {
    //             let mut task_map = HashMap::new();
    //             task_map.insert("name".to_string(), AttributeValue::S(task.name.clone()));
    //             task_map.insert(
    //                 "category".to_string(),
    //                 AttributeValue::S(task.category.clone()),
    //             );
    //             task_map.insert(
    //                 "startTime".to_string(),
    //                 AttributeValue::S(task.start_time.clone()),
    //             );
    //             task_map.insert(
    //                 "endTime".to_string(),
    //                 AttributeValue::S(task.end_time.clone()),
    //             );
    //             task_map.insert(
    //                 "taskId".to_string(),
    //                 AttributeValue::S(task.task_id.clone()),
    //             );
    //             task_map.insert(
    //                 "completed".to_string(),
    //                 AttributeValue::Bool(task.completed),
    //             );
    //             task_map.insert(
    //                 "createdAt".to_string(),
    //                 AttributeValue::S(task.created_at.clone()),
    //             );
    //             task_map.insert(
    //                 "updatedAt".to_string(),
    //                 AttributeValue::S(task.updated_at.clone()),
    //             );
    //             AttributeValue::M(task_map)
    //         })
    //         .collect();

    //     item.insert("tasks".to_string(), AttributeValue::L(tasks));

    //     Ok(item)
    // }
}
