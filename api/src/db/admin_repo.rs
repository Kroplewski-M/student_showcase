use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::dtos::admin::{ChartData, Dashboard, FindStudent};

#[derive(Clone)]
pub struct AdminRepo {
    pool: Pool<Postgres>,
}

impl AdminRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
pub trait AdminRepoTrait: Send + Sync {
    async fn search_student(&self, id: &str) -> Result<Option<FindStudent>, sqlx::Error>;
    async fn suspend_student(&self, id: &str) -> Result<(), sqlx::Error>;
    async fn unsuspend_student(&self, id: &str) -> Result<(), sqlx::Error>;
    async fn get_dashboard_data(&self) -> Result<Dashboard, sqlx::Error>;
}

#[async_trait]
impl AdminRepoTrait for AdminRepo {
    async fn search_student(&self, id: &str) -> Result<Option<FindStudent>, sqlx::Error> {
        sqlx::query_as!(
            FindStudent,
            r#"
            SELECT 
            u.id, 
            u.suspended,
            f.new_file_name || '.' || f.extension AS image_name
            FROM users u
            LEFT JOIN files f ON u.image_id = f.Id
            WHERE u.is_admin = false
            AND u.id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }
    async fn suspend_student(&self, id: &str) -> Result<(), sqlx::Error> {
        let res = sqlx::query!(
            r#"
            UPDATE users
            SET suspended = true
            WHERE id = $1
            AND is_admin = false
        "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if res.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }
    async fn unsuspend_student(&self, id: &str) -> Result<(), sqlx::Error> {
        let res = sqlx::query!(
            r#"
            UPDATE users
            SET suspended = false 
            WHERE id = $1
            AND is_admin = false
        "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if res.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }

    async fn get_dashboard_data(&self) -> Result<Dashboard, sqlx::Error> {
        let students_verified = sqlx::query_as!(
            ChartData,
            r#"
            SELECT
                CASE WHEN verified THEN 'Verified' ELSE 'Unverified' END as "name!",
                COUNT(*)::int as "value!"
            FROM users
            WHERE id NOT LIKE '0%'
            GROUP BY verified
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let student_interests = sqlx::query_as!(
            ChartData,
            r#"
            SELECT t.name, COUNT(*)::int as "value!"
            FROM user_tools si
            JOIN software_tools t ON t.id = si.software_tool_id
            GROUP BY t.id, t.name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let student_courses = sqlx::query_as!(
            ChartData,
            r#"
            SELECT c.name, COUNT(*)::int as "value!" 
            FROM users u
            JOIN courses c ON c.Id = u.course_id
            GROUP BY c.Id, c.name
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        let project_stack = sqlx::query_as!(
            ChartData,
            r#"
                SELECT st.name, COUNT(*)::int as "value!"
                FROM project_tools pt
                JOIN software_tools st ON pt.tool_id = st.id
                GROUP BY st.id, st.name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let students_with_project = sqlx::query_as!(
            ChartData,
            r#"
            SELECT
                CASE WHEN project_count > 0 THEN 'Atleast 1 Project'
                ELSE 'No Projects' END as "name!",
                COUNT(*)::int as "value!"
            FROM (
                SELECT u.id, COUNT(p.id) as project_count
                FROM users u
                LEFT JOIN projects p ON p.user_id = u.id
                GROUP BY u.id
            ) counts
            GROUP BY project_count > 0
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(Dashboard {
            students_verified,
            student_interests,
            student_courses,
            project_stack,
            students_with_project,
        })
    }
}
