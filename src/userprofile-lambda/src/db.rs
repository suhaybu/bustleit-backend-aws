use aws_sdk_dynamodb::types::AttributeValue;

use common::{
    dynamodb::DynamoDbClient,
    error::{Error, Result},
    models::dynamodb::{Scores, UserProfileDB},
};
use std::collections::HashMap;

pub struct UserProfileDb {
    db: DynamoDbClient,
}

impl UserProfileDb {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            db: DynamoDbClient::new().await?,
        })
    }

    pub async fn get_user_profile(&self, user_id: String) -> Result<UserProfileDB> {
        let pk = format!("USER#{}", user_id);
        let sk = "PROFILE".to_string();

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
                    AttributeValue::S(DynamoDbClient::create_user_pk(&id.to_string())),
                );
                key.insert(
                    "SK".to_string(),
                    AttributeValue::S(DynamoDbClient::create_profile_sk()),
                );
                key
            })
            .collect();

        let keys_and_attributes = aws_sdk_dynamodb::types::KeysAndAttributes::builder()
            .set_keys(Some(keys))
            .build()
            .map_err(|e| Error::db_query_error(e.to_string()))?;

        let result = self
            .db
            .client
            .batch_get_item()
            .request_items(&self.db.table_name, keys_and_attributes)
            .send()
            .await
            .map_err(|e| Error::db_query_error(e.to_string()))?;

        match result.responses {
            Some(items) => {
                let profiles = items
                    .get(&self.db.table_name)
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
            .db
            .client
            .query()
            .table_name(&self.db.table_name)
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
        let mut all_profiles = Vec::new();
        let mut exclusive_start_key: Option<HashMap<String, AttributeValue>> = None;

        loop {
            // tracing::debug!("\nScanning for user profiles");
            let mut scan = self
                .db
                .client
                .scan()
                .table_name(&self.db.table_name)
                .filter_expression("SK = :profile")
                .expression_attribute_values(":profile", AttributeValue::S("PROFILE".to_string()));

            if let Some(last_key) = &exclusive_start_key {
                for (k, v) in last_key {
                    scan = scan.exclusive_start_key(k.clone(), v.clone());
                }
            }

            let result = scan
                .send()
                .await
                .map_err(|e| Error::db_query_error(e.to_string()))?;

            if let Some(items) = result.items {
                // tracing::debug!(batch_size = items.len(), "Retrieved batch of profiles");

                let profiles: Vec<Result<UserProfileDB>> = items
                    .into_iter()
                    .map(|item| self.convert_to_user_profile(&item))
                    .collect();

                // Filter out any conversion errors and collect successful conversions
                all_profiles.extend(profiles.into_iter().filter_map(|result| result.ok()));
            }

            // Check if we need to continue scanning
            exclusive_start_key = result.last_evaluated_key;

            if exclusive_start_key.is_none() {
                break;
            }
        }

        if all_profiles.is_empty() {
            Err(Error::not_found("UserProfiles", "No profiles found"))
        } else {
            Ok(all_profiles)
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
    fn extract_number(&self, attr: Option<&AttributeValue>) -> Result<f32> {
        match attr {
            Some(AttributeValue::N(num_str)) => num_str
                .parse::<f32>()
                .map_err(|e| Error::db_parse_error(format!("Failed to parse number: {}", e))),
            _ => Ok(0.0),
        }
    }
}
