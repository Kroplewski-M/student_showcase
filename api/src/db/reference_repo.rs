use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::dtos::reference::{Course, LinkType, SiteInfo, SoftwareTool};

#[cfg(test)]
pub mod mocks {
    use crate::dtos::reference::SiteInfo;

    use super::*;
    use mockall::mock;

    mock! {
        pub ReferenceRepo {}

        #[async_trait]
        impl ReferenceRepoTrait for ReferenceRepo {
            async fn get_link_types(&self) -> Result<Vec<LinkType>, sqlx::Error>;
            async fn get_courses(&self) -> Result<Vec<Course>, sqlx::Error>;
            async fn get_tools(&self) -> Result<Vec<SoftwareTool>, sqlx::Error>;
            async fn get_site_info(&self) -> Result<SiteInfo, sqlx::Error>;
        }
    }
}

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
    async fn get_tools(&self) -> Result<Vec<SoftwareTool>, sqlx::Error>;
    async fn get_site_info(&self) -> Result<SiteInfo, sqlx::Error>;
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
    async fn get_tools(&self) -> Result<Vec<SoftwareTool>, sqlx::Error> {
        sqlx::query_as!(
            SoftwareTool,
            r#"
            SELECT t.id, t.name FROM software_tools t 
            ORDER BY t.name
        "#
        )
        .fetch_all(&self.pool)
        .await
    }
    async fn get_site_info(&self) -> Result<SiteInfo, sqlx::Error> {
        let info = sqlx::query!(
            r#"
            SELECT
            (SELECT COUNT(*) FROM users WHERE verified = true) AS student_count,
            (SELECT COUNT(*) FROM projects) AS project_count
        "#
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(SiteInfo {
            student_count: info.student_count.unwrap_or(0),
            project_count: info.project_count.unwrap_or(0),
        })
    }
}
