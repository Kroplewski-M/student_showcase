use serde::Serialize;

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct FindStudent {
    pub id: String,
    pub image_name: Option<String>,
    pub suspended: bool,
}
