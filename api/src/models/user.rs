use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub personal_email: Option<String>,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip)]
    pub password: Option<String>,
    pub is_admin: bool,
}
#[derive(Debug, Serialize, sqlx::FromRow, Clone)]
pub struct AuthUser {
    pub id: String,
    pub verified: bool,
    pub is_admin: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ProjectBaseRow {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub live_link: Option<String>,
}
