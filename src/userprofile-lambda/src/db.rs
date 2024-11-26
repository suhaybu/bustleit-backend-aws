use sqlx::{postgres::PgRow, PgPool, Row};
use uuid::Uuid;

use common::{
    database::DatabaseConfig,
    error::{Error, Result},
    models::database as DB,
};

pub struct ProfileDb {
    pool: PgPool,
}

impl ProfileDb {
    pub async fn new() -> Result<Self> {
        let config = DatabaseConfig::new()?;
        let pool = common::database::create_pool(config).await?;

        Ok(Self { pool })
    }

    // Get single user profile
    pub async fn get_profile(&self, user_id: Uuid) -> Result<DB::Profile> {
        let row = sqlx::query(
            "SELECT user_id, cluster, preferences, personality_scores, created_at, updated_at
                 FROM profiles
                 WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::not_found(user_id.to_string()),
            e => Error::Database(e),
        })?;

        let profile = Self::map_profile_row(row)?;

        Ok(profile)
    }

    // Get multiple user profiles
    pub async fn get_profiles(&self, user_ids: &[Uuid]) -> Result<Vec<DB::Profile>> {
        let rows = sqlx::query(
            "SELECT user_id, cluster, preferences, personality_scores, created_at, updated_at
                 FROM profiles
                 WHERE user_id = ANY($1)",
        )
        .bind(user_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        let profiles = rows
            .into_iter()
            .map(Self::map_profile_row)
            .collect::<Result<Vec<_>>>()?;

        Ok(profiles)
    }

    pub async fn get_profiles_by_cluster(&self, cluster: i32) -> Result<Vec<DB::Profile>> {
        let rows = sqlx::query(
            "SELECT user_id, cluster, preferences, personality_scores, created_at, updated_at
                 FROM profiles
                 WHERE cluster = $1",
        )
        .bind(cluster)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        if rows.is_empty() {
            return Err(Error::not_found(cluster.to_string()));
        }

        let profiles = rows
            .into_iter()
            .map(Self::map_profile_row)
            .collect::<Result<Vec<_>>>()?;

        Ok(profiles)
    }

    pub async fn get_all_users(&self) -> Result<Vec<DB::Profile>> {
        let rows = sqlx::query(
            "SELECT user_id, cluster, preferences, personality_scores, created_at, updated_at
                 FROM profiles",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e))?;

        if rows.is_empty() {
            return Err(Error::not_found("No profiles found".to_string()));
        }

        let profiles = rows
            .into_iter()
            .map(Self::map_profile_row)
            .collect::<Result<Vec<_>>>()?;

        Ok(profiles)
    }

    fn map_profile_row(row: PgRow) -> Result<DB::Profile> {
        Ok(DB::Profile {
            user_id: row.get("user_id"),
            cluster: row.get("cluster"),
            preferences: row.get("preferences"),
            personality_scores: row.get("personality_scores"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}
