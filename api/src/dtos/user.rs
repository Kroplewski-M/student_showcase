use actix_multipart::form::{MultipartForm, json::Json, tempfile::TempFile};
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::dtos::reference::{Course, FileInfo, LinkType, SoftwareTool};

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
    pub featured_project_id: Option<Uuid>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct ProjToolRow {
    pub project_id: Uuid,
    pub name: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct ProjLinkRow {
    pub project_id: Uuid,
    pub id: Uuid,
    pub link_type: String,
    pub url: String,
    pub name: Option<String>,
}
#[derive(sqlx::FromRow, Debug)]
pub struct ProjImageRow {
    pub project_id: Uuid,
    pub file_id: Uuid,
    pub file_name: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectProfileViewBase {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub live_link: Option<String>,
    pub featured_img_id: Option<Uuid>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectImageView {
    pub file_id: Uuid,
    pub file_name: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectProfileView {
    #[serde(flatten)]
    pub base: ProjectProfileViewBase,
    pub tools: Vec<String>,
    pub images: Vec<ProjectImageView>,
    pub links: Vec<UserLinkView>,
}
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileView {
    #[serde(flatten)]
    pub base: UserProfileRowView,
    pub certificates: Vec<String>,
    pub tools: Vec<String>,
    pub links: Vec<UserLinkView>,
    pub projects: Vec<ProjectProfileView>,
}

//Used to get the form to edit profile
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

//patch user info form
#[derive(Debug, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpsertLinkPayload {
    pub link_type_id: Uuid,
    #[validate(length(max = 50))]
    pub name: Option<String>,
    #[validate(url)]
    pub url: String,
}
#[derive(Debug, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserInfo {
    #[validate(required, length(min = 1, max = 50))]
    pub first_name: Option<String>,
    #[validate(required, length(min = 1, max = 50))]
    pub last_name: Option<String>,
    pub personal_email: Option<String>,
    pub description: Option<String>,
    #[validate(required)]
    pub selected_course: Option<Uuid>,
    #[validate(nested)]
    pub links: Vec<UpsertLinkPayload>,
    pub certificates: Vec<String>,
    pub selected_tools: Vec<Uuid>,
}

impl UpdateUserInfo {
    pub fn to_embedding_document(
        &self,
        course_name: Option<&str>,
        tool_names: &[String],
    ) -> String {
        let mut parts: Vec<String> = Vec::new();

        let name = match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
            _ => None,
        };

        if let (Some(name), Some(course)) = (&name, course_name) {
            parts.push(format!("{} is studying {}", name, course));
        }

        if let Some(desc) = &self.description
            && !desc.trim().is_empty()
        {
            parts.push(desc.clone());
        }

        if !tool_names.is_empty() {
            let tools = tool_names.join(", ");
            let subject = name.as_deref().unwrap_or("They");
            if tool_names.len() == 1 {
                parts.push(format!("{} has an interest in {}", subject, tools));
            } else {
                parts.push(format!("{} has interests in {}", subject, tools));
            }
        }

        if !self.certificates.is_empty() {
            parts.push(format!(
                "They hold the following certificates: {}",
                self.certificates.join(", ")
            ));
        }

        parts.join(". ")
    }
}

//used to get the form to upsert project

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectFormData {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub live_link: Option<String>,
    pub links: Vec<UserLinkView>,
    pub selected_tools: Vec<Uuid>,
    pub existing_images: Vec<String>,
}
impl Default for ProjectFormData {
    fn default() -> Self {
        Self::new()
    }
}
impl ProjectFormData {
    pub fn new() -> Self {
        Self {
            id: None,
            name: "".to_string(),
            description: "".to_string(),
            live_link: None,
            links: vec![],
            selected_tools: vec![],
            existing_images: vec![],
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectForm {
    #[serde(flatten)]
    pub project: ProjectFormData,
    pub link_types: Vec<LinkType>,
    pub tools_list: Vec<SoftwareTool>,
}
#[derive(Debug, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUpsertData {
    pub id: Option<Uuid>,
    #[validate(length(min = 1, max = 250))]
    pub name: String,
    pub description: String,
    pub live_link: Option<String>,
    pub links: Vec<UpsertLinkPayload>,
    pub selected_tools: Vec<Uuid>,
    pub existing_images: Vec<String>,
}
impl ProjectUpsertData {
    pub fn to_embedding_document(&self, tool_names: &[String]) -> String {
        let mut parts: Vec<String> = Vec::new();

        parts.push(format!("{} is a project", self.name));

        if !self.description.trim().is_empty() {
            parts.push(self.description.clone());
        }

        if !tool_names.is_empty() {
            let tools = tool_names.join(", ");
            if tool_names.len() == 1 {
                parts.push(format!("It uses {}", tools));
            } else {
                parts.push(format!("It uses the following tools: {}", tools));
            }
        }

        if let Some(link) = &self.live_link
            && !link.trim().is_empty()
        {
            parts.push(format!("It is live at {}", link));
        }

        parts.join(". ")
    }
}

#[derive(Debug, MultipartForm)]
pub struct ProjectFormUpsert {
    pub data: Json<ProjectUpsertData>,
    pub new_files: Vec<TempFile>,
}
#[derive(Deserialize)]
pub struct UpsertProjectQuery {
    pub project_id: Option<Uuid>,
}
pub struct UpsertProjectParams {
    pub user_id: String,
    pub project_id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub live_link: Option<String>,
    pub selected_tools: Vec<Uuid>,
    pub links: Vec<UpsertLinkPayload>,
    pub new_images: Vec<FileInfo>,
    pub existing_images: Vec<String>,
    pub embedding: Vector,
}
