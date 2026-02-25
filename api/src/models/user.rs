use chrono::prelude::*;
use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserLink {
    pub link_type: String,
    pub link_url: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileRow {
    pub id: String,
    pub profile_image_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub personal_email: Option<String>,
    pub course_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    #[serde(flatten)]
    pub base: UserProfileRow,
    pub certificates: Vec<String>,
    pub tools: Vec<String>,
    pub links: Vec<UserLink>,
}
