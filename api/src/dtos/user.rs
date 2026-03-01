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

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileView {
    #[serde(flatten)]
    pub base: UserProfileRowView,
    pub certificates: Vec<String>,
    pub tools: Vec<String>,
    pub links: Vec<UserLinkView>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserFormData {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub personal_email: Option<String>,
    pub description: Option<String>,
    pub selected_course: Option<Uuid>,
    pub links: Vec<UserLinkView>,
    pub certificates: Vec<String>,
    pub selected_tools: Vec<Uuid>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileForm {
    #[serde(flatten)]
    pub user_data: UserFormData,
    pub courses_list: Vec<Course>,
    pub link_types: Vec<LinkType>,
    pub tools_list: Vec<SoftwareTool>,
}
