use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::User;

#[derive(Debug, Clone)]
pub struct AuthRepo {
    pool: Pool<Postgres>,
}

impl AuthRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn exists_verified(&self, student_id: &str) -> Result<bool, sqlx::Error> {
        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
            SELECT 1
            FROM users
            WHERE id = $1
            AND verified = true
        )
        "#,
            student_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists.unwrap_or(false))
    }
    pub async fn get_user_by_id(&self, student_id: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT
            id,
            first_name,
            last_name,
            personal_email,
            verified,
            created_at,
            updated_at,
            password
            FROM users WHERE id = $1"#,
            student_id
        )
        .fetch_optional(&self.pool)
        .await
    }
    pub async fn create_user(
        &self,
        student_id: &str,
        password: &str,
    ) -> Result<String, sqlx::Error> {
        let user_id = sqlx::query_scalar!(
            r#"
            INSERT INTO users (id, password)
            VALUES ($1,$2)
            RETURNING id
            "#,
            student_id,
            password
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user_id)
    }
    pub async fn create_user_verification(&self, student_id: &str) -> Result<Uuid, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        //delete all prev tokens for this user, so theres only one active one
        sqlx::query!(
            "DELETE FROM user_verifications WHERE user_id = $1",
            student_id
        )
        .execute(tx.as_mut())
        .await?;

        let token = sqlx::query_scalar!(
            r#"
            INSERT INTO user_verifications (token, user_id, expired_at)
            VALUES ($1, $2, now() + interval '15 minutes')
            RETURNING token
            "#,
            Uuid::new_v4(),
            student_id
        )
        .fetch_one(tx.as_mut())
        .await?;
        tx.commit().await?;
        Ok(token)
    }
    pub async fn create_user_reset_password(&self, student_id: &str) -> Result<Uuid, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let user_exists = self.exists_verified(student_id).await?;
        if !user_exists {
            return Err(sqlx::Error::RowNotFound);
        }
        //delete all prev tokens for user so theres only one active
        sqlx::query!(
            "DELETE FROM user_password_resets WHERE user_id = $1",
            student_id
        )
        .execute(tx.as_mut())
        .await?;

        let token = sqlx::query_scalar!(
            r#"
        INSERT INTO user_password_resets (token, user_id, expired_at)
        VALUES ($1, $2, now() + interval '15 minutes')
        RETURNING token
        "#,
            Uuid::new_v4(),
            student_id
        )
        .fetch_one(tx.as_mut())
        .await?;
        tx.commit().await?;
        Ok(token)
    }
    pub async fn user_reset_password_exists(&self, token: Uuid) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
            SELECT 1 FROM user_password_resets
            WHERE token = $1
            AND expired_at > now()
            ) as "exists!: bool"
            "#,
            token
        )
        .fetch_one(&self.pool)
        .await
    }
    pub async fn update_user_password(
        &self,
        token: Uuid,
        password: &str,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let user_id = sqlx::query_scalar!(
            r#"DELETE FROM user_password_resets
            WHERE token = $1
            AND expired_at > now()
            RETURNING user_id"#,
            token
        )
        .fetch_optional(tx.as_mut())
        .await?;
        if user_id.is_none() {
            tx.rollback().await?;
            return Err(sqlx::Error::RowNotFound);
        }
        sqlx::query!(
            r#"
            UPDATE users
            SET password = $1
            WHERE id = $2
            "#,
            password,
            user_id.unwrap(),
        )
        .execute(tx.as_mut())
        .await?;
        tx.commit().await?;
        Ok(())
    }
    pub async fn validate_user(&self, token: Uuid) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let student_id: Option<String> = sqlx::query_scalar!(
            r#"
            DELETE FROM user_verifications
            WHERE token = $1
            AND expired_at > now()
            RETURNING user_id
            "#,
            token,
        )
        .fetch_optional(tx.as_mut())
        .await?;
        if student_id.is_none() {
            tx.rollback().await?;
            return Err(sqlx::Error::RowNotFound);
        }
        sqlx::query!(
            r#"
            UPDATE users
            SET verified = true
            WHERE id = $1
            "#,
            student_id.unwrap()
        )
        .execute(tx.as_mut())
        .await?;

        tx.commit().await?;
        Ok(())
    }
}
