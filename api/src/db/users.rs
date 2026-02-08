use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct UsersRepo {
    pool: Pool<Postgres>,
}

impl UsersRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn exists_verified(&self, user_id: &String) -> Result<bool, sqlx::Error> {
        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
            SELECT 1
            FROM users
            WHERE id = $1
            AND verified = true
        )
        "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists.unwrap_or(false))
    }
}
