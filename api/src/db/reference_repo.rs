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
        let counts = sqlx::query!(
            r#"
            SELECT
            (SELECT COUNT(*) FROM users WHERE verified = true AND id NOT LIKE '0%') AS student_count,
            (SELECT COUNT(*) FROM projects WHERE user_id NOT LIKE '0%') AS project_count
        "#
        )
        .fetch_one(&self.pool)
        .await?;

        let top_interests = sqlx::query_scalar!(
            r#"
            SELECT st.name
            FROM software_tools st
            INNER JOIN
            (
                SELECT tool_id FROM project_tools
                UNION ALL
                SELECT software_tool_id FROM user_tools
            ) combined ON st.id = combined.tool_id
            GROUP BY st.id, st.name
            ORDER BY COUNT(combined.tool_id) DESC
            LIMIT 10
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(SiteInfo {
            student_count: counts.student_count.unwrap_or(0),
            project_count: counts.project_count.unwrap_or(0),
            top_interests,
        })
    }
}
