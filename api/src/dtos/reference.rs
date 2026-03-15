use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LinkType {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Course {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SoftwareTool {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug)]
pub struct FileInfo {
    pub old_name: String,
    pub new_name: String,
    pub extension: String,
    pub length: i64,
    pub file_type: String,
}
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SiteInfo {
    pub student_count: i64,
    pub project_count: i64,
}
