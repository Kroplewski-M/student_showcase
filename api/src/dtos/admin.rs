use serde::Serialize;

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct FindStudent {
    pub id: String,
    pub image_name: Option<String>,
    pub suspended: bool,
}

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct ChartData {
    pub name: String,
    pub value: i32,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dashboard {
    pub students_verified: Vec<ChartData>,
    pub student_interests: Vec<ChartData>,
    pub student_courses: Vec<ChartData>,
    pub project_stack: Vec<ChartData>,
    pub students_with_project: Vec<ChartData>,
}
