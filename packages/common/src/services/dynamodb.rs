use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use std::collections::HashMap;
use std::env;

use crate::error::{Error, Result};
use crate::models::dynamodb::{Scores, Task, UserProfileDB, UserTasks};

pub struct DynamoDbClient {
    client: Client,
    table_name: String,
}

#[allow(dead_code)]
impl DynamoDbClient {
    pub async fn new() -> Result<Self> {
        // Loads env
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&config);

        let table_name = env::var("DYNAMODB_TABLE").map_err(|e| {
            Error::db_connection_failed(format!("Missing DYNAMODB_TABLE env var: {}", e))
        })?;

        Ok(Self { client, table_name })
    }

    // Creating Keys
    // pk = Primary Key, sk = Secondary KeyZ
    fn create_user_pk(user_id: &str) -> String {
        format!("USER#{}", user_id)
    }

    fn create_task_pk(date: &str, task_id: &str) -> String {
        format!("TASK#DATE#{}#{}", date, task_id)
    }

    fn create_stats_pk(user_id: &str) -> String {
        format!("STATS#USER#{}", user_id)
    }

    fn create_profile_sk() -> String {
        "PROFILE".to_string()
    }

    fn create_task_sk(date: &str) -> String {
        format!("TASK#DATE#{}", date)
    }

    // Read Operations

    pub async fn get_user_profile(&self, user_id: String) -> Result<UserProfileDB> {
        let pk = format!("USER#{}", user_id);
        let sk = "PROFILE".to_string();

        let result = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("PK", AttributeValue::S(pk))
            .key("SK", AttributeValue::S(sk))
            .send()
            .await
            .map_err(|e| Error::db_query_error(e.to_string()))?;

        match result.item {
            Some(item) => self.convert_to_user_profile(&item),
            None => Err(Error::not_found("UserProfile", user_id)),
        }
    }

    pub async fn get_user_profiles(&self, user_ids: Vec<String>) -> Result<Vec<UserProfileDB>> {
        let keys: Vec<HashMap<String, AttributeValue>> = user_ids
            .iter()
            .map(|id| {
                let mut key = HashMap::new();
                key.insert(
                    "PK".to_string(),
                    AttributeValue::S(Self::create_user_pk(&id.to_string())),
                );
                key.insert(
                    "SK".to_string(),
                    AttributeValue::S(Self::create_profile_sk()),
                );
                key
            })
            .collect();

        let keys_and_attributes = aws_sdk_dynamodb::types::KeysAndAttributes::builder()
            .set_keys(Some(keys))
            .build()
            .map_err(|e| Error::db_query_error(e.to_string()))?;

        let result = self
            .client
            .batch_get_item()
            .request_items(&self.table_name, keys_and_attributes)
            .send()
            .await
            .map_err(|e| Error::db_query_error(e.to_string()))?;

        match result.responses {
            Some(items) => {
                let profiles = items
                    .get(&self.table_name)
                    .map(|items| {
                        items
                            .iter()
                            .filter_map(|item| self.convert_to_user_profile(item).ok())
                            .collect()
                    })
                    .unwrap_or_default();
                Ok(profiles)
            }
            None => Ok(Vec::new()),
        }
    }

    pub async fn get_users_by_cluster(&self, cluster: i32) -> Result<Vec<UserProfileDB>> {
        let gsi1pk = format!("CLUSTER#{}", cluster);

        let result = self
            .client
            .query()
            .table_name(&self.table_name)
            .index_name("GSI1")
            .key_condition_expression("GSI1PK = :cluster")
            .expression_attribute_values(":cluster", AttributeValue::S(gsi1pk))
            .send()
            .await
            .map_err(|e| Error::db_query_error(e.to_string()))?;

        match result.items {
            Some(items) => {
                let profiles: Vec<Result<UserProfileDB>> = items
                    .into_iter()
                    .map(|item| self.convert_to_user_profile(&item))
                    .collect();

                // Filter out any conversion errors and collect successful conversions
                let valid_profiles: Vec<UserProfileDB> = profiles
                    .into_iter()
                    .filter_map(|result| result.ok())
                    .collect();

                if valid_profiles.is_empty() {
                    Err(Error::not_found(
                        "UserProfiles",
                        format!("cluster {}", cluster),
                    ))
                } else {
                    Ok(valid_profiles)
                }
            }
            None => Err(Error::not_found(
                "UserProfiles",
                format!("cluster {}", cluster),
            )),
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserProfileDB>> {
        let result = self
            .client
            .scan()
            .table_name(&self.table_name)
            .filter_expression("SK = :profile")
            .expression_attribute_values(":profile", AttributeValue::S("PROFILE".to_string()))
            .send()
            .await
            .map_err(|e| Error::db_query_error(e.to_string()))?;

        match result.items {
            Some(items) => {
                let profiles: Vec<Result<UserProfileDB>> = items
                    .into_iter()
                    .map(|item| self.convert_to_user_profile(&item))
                    .collect();

                // Filter out any conversion errors and collect successful conversions
                let valid_profiles: Vec<UserProfileDB> = profiles
                    .into_iter()
                    .filter_map(|result| result.ok())
                    .collect();

                if valid_profiles.is_empty() {
                    Err(Error::not_found("UserProfiles", "No profiles found"))
                } else {
                    Ok(valid_profiles)
                }
            }
            None => Err(Error::not_found("UserProfiles", "No users found")),
        }
    }

    pub async fn get_user_tasks(
        &self,
        user_id: &str,
        date: &str,
        date_end: Option<&str>,
    ) -> Result<Vec<UserTasks>> {
        if let Some(date_end) = date_end {
            self.get_user_tasks_range(user_id, date, date_end).await
        } else {
            let tasks = self.get_user_tasks_single_day(user_id, date).await?;
            Ok(vec![tasks])
        }
    }

    async fn get_user_tasks_single_day(&self, user_id: &str, date: &str) -> Result<UserTasks> {
        let pk = format!("USER#{}", user_id);
        let sk = format!("TASK#DATE#{}", date);

        let result = self
            .client
            .get_item()
            .table_name(&self.table_name)
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

    async fn get_user_tasks_range(
        &self,
        user_id: &str,
        date_start: &str,
        date_end: &str,
    ) -> Result<Vec<UserTasks>> {
        let pk = Self::create_user_pk(user_id);
        let start_sk = Self::create_task_sk(date_start);
        let end_sk = Self::create_task_sk(date_end);

        let result = self
            .client
            .query()
            .table_name(&self.table_name)
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

    // Conversion Helpers
    fn convert_to_user_profile(
        &self,
        item: &HashMap<String, AttributeValue>,
    ) -> Result<UserProfileDB> {
        let preferences = if let Some(AttributeValue::L(prefs)) = item.get("preferences") {
            prefs
                .iter()
                .filter_map(|av| {
                    if let AttributeValue::S(s) = av {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        };

        let scores = if let Some(AttributeValue::M(scores_map)) = item.get("scores") {
            Scores {
                turbulent: self.extract_number(scores_map.get("turbulent"))?,
                introverted: self.extract_number(scores_map.get("introverted"))?,
                prospecting: self.extract_number(scores_map.get("prospecting"))?,
                judging: self.extract_number(scores_map.get("judging"))?,
                assertive: self.extract_number(scores_map.get("assertive"))?,
                feeling: self.extract_number(scores_map.get("feeling"))?,
                extraverted: self.extract_number(scores_map.get("extraverted"))?,
                observant: self.extract_number(scores_map.get("observant"))?,
                thinking: self.extract_number(scores_map.get("thinking"))?,
                intuitive: self.extract_number(scores_map.get("intuitive"))?,
            }
        } else {
            return Err(Error::db_parse_error("Missing scores map"));
        };

        let cluster = self
            .extract_number(item.get("cluster"))
            .map(|n| n as i32)
            .unwrap_or(0);

        Ok(UserProfileDB::new(
            // Extract UUID from PK (remove "USER#" prefix)
            &item
                .get("PK")
                .and_then(|v| v.as_s().ok())
                .map(|s| s.trim_start_matches("USER#").to_string())
                .unwrap_or_default(),
            cluster,
            preferences,
            scores,
        ))
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

    fn convert_from_user_tasks(
        &self,
        user_tasks: &UserTasks,
    ) -> Result<HashMap<String, AttributeValue>> {
        let mut item = HashMap::new();

        // Add keys
        item.insert("PK".to_string(), AttributeValue::S(user_tasks.pk.clone()));
        item.insert("SK".to_string(), AttributeValue::S(user_tasks.sk.clone()));
        item.insert(
            "GSI1PK".to_string(),
            AttributeValue::S(user_tasks.gsi1pk.clone()),
        );
        item.insert(
            "GSI1SK".to_string(),
            AttributeValue::S(user_tasks.gsi1sk.clone()),
        );

        // Add metadata
        item.insert(
            "date".to_string(),
            AttributeValue::S(user_tasks.date.clone()),
        );
        item.insert(
            "completedTasks".to_string(),
            AttributeValue::N(user_tasks.completed_tasks.to_string()),
        );
        item.insert(
            "totalTasks".to_string(),
            AttributeValue::N(user_tasks.total_tasks.to_string()),
        );

        // Convert tasks to AttributeValue
        let tasks: Vec<AttributeValue> = user_tasks
            .tasks
            .iter()
            .map(|task| {
                let mut task_map = HashMap::new();
                task_map.insert("name".to_string(), AttributeValue::S(task.name.clone()));
                task_map.insert(
                    "category".to_string(),
                    AttributeValue::S(task.category.clone()),
                );
                task_map.insert(
                    "startTime".to_string(),
                    AttributeValue::S(task.start_time.clone()),
                );
                task_map.insert(
                    "endTime".to_string(),
                    AttributeValue::S(task.end_time.clone()),
                );
                task_map.insert(
                    "taskId".to_string(),
                    AttributeValue::S(task.task_id.clone()),
                );
                task_map.insert(
                    "completed".to_string(),
                    AttributeValue::Bool(task.completed),
                );
                task_map.insert(
                    "createdAt".to_string(),
                    AttributeValue::S(task.created_at.clone()),
                );
                task_map.insert(
                    "updatedAt".to_string(),
                    AttributeValue::S(task.updated_at.clone()),
                );
                AttributeValue::M(task_map)
            })
            .collect();

        item.insert("tasks".to_string(), AttributeValue::L(tasks));

        Ok(item)
    }

    fn extract_number(&self, attr: Option<&AttributeValue>) -> Result<f32> {
        match attr {
            Some(AttributeValue::N(num_str)) => num_str
                .parse::<f32>()
                .map_err(|e| Error::db_parse_error(format!("Failed to parse number: {}", e))),
            _ => Ok(0.0),
        }
    }
}
