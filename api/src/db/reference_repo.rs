use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::dtos::reference::{Course, LinkType};

#[derive(Debug, Clone)]
pub struct ReferenceRepo {
    pool: Pool<Postgres>,
}

impl ReferenceRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
pub trait ReferenceRepoTrait: Send + Sync {
    async fn get_link_types(&self) -> Result<Vec<LinkType>, sqlx::Error>;
    async fn get_courses(&self) -> Result<Vec<Course>, sqlx::Error>;
}

#[async_trait]
impl ReferenceRepoTrait for ReferenceRepo {
    async fn get_link_types(&self) -> Result<Vec<LinkType>, sqlx::Error> {
        sqlx::query_as!(
            LinkType,
            r#"
            SELECT l.id, l.name FROM link_types l
            ORDER BY l.name
        "#
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn get_courses(&self) -> Result<Vec<Course>, sqlx::Error> {
        sqlx::query_as!(
            Course,
            r#"
            SELECT c.id, c.name FROM courses c
            ORDER BY c.name
        "#
        )
        .fetch_all(&self.pool)
        .await
    }
}
