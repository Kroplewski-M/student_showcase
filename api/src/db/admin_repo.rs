use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::dtos::admin::FindStudent;

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
        let res = sqlx::query_scalar!(
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
        let res = sqlx::query_scalar!(
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
}
