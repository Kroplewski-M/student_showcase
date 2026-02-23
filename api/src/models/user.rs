use chrono::prelude::*;
use serde::Serialize;

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

#[derive(Debug, Serialize, sqlx::FromRow, Clone)]
pub struct UserProfile {
    pub id: String,
    pub profile_image_name: Option<String>,
}
