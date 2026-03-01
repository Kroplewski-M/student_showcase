use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dtos::reference::{Course, LinkType, SoftwareTool};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserLinkView {
    pub id: Uuid,
    pub link_type: String,
    pub url: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileRowView {
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
pub struct UserProfileView {
    #[serde(flatten)]
    pub base: UserProfileRowView,
    pub certificates: Vec<String>,
    pub tools: Vec<String>,
    pub links: Vec<UserLinkView>,
}

pub struct UserProfileForm {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub personal_email: Option<String>,
    pub description: Option<String>,
    pub courses_list: Vec<Course>,
    pub selected_course: Option<Uuid>,
    pub link_types: Vec<LinkType>,
    pub links: Vec<UserLinkView>,
    pub certificates: Vec<String>,
    pub tools_list: Vec<SoftwareTool>,
    pub selected_tools: Vec<Uuid>,
}
