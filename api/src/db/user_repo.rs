use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use tracing::info;

use crate::models::{
    file::File,
    user::{User, UserProfile},
};

#[derive(Debug, Clone)]
pub struct UserRepo {
    pool: Pool<Postgres>,
}

impl UserRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
pub trait UserRepoTrait: Send + Sync {
    async fn exists_verified(&self, student_id: &str) -> Result<bool, sqlx::Error>;
    async fn get_user_by_id(&self, student_id: &str) -> Result<Option<User>, sqlx::Error>;
    async fn update_user_image(
        &self,
        user_id: &str,
        file_size: i64,
        image_type: &str,
        old_name: &str,
        new_name: &str,
        extension: &str,
    ) -> Result<(), sqlx::Error>;
    async fn get_user_image(&self, user_id: &str) -> Result<Option<File>, sqlx::Error>;
    async fn get_user_profile(&self, user_id: &str) -> Result<UserProfile, sqlx::Error>;
}

#[async_trait]
impl UserRepoTrait for UserRepo {
    async fn exists_verified(&self, student_id: &str) -> Result<bool, sqlx::Error> {
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
    async fn get_user_by_id(&self, student_id: &str) -> Result<Option<User>, sqlx::Error> {
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
    async fn update_user_image(
        &self,
        user_id: &str,
        file_size: i64,
        image_type: &str,
        old_name: &str,
        new_name: &str,
        extension: &str,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        //delete old image if it exists
        let old_id = sqlx::query_scalar!(
            r#"
            WITH old AS (
                SELECT image_id FROM users WHERE id = $1
            )
            UPDATE users
            SET image_id = NULL
            WHERE id = $1
            RETURNING (SELECT image_id FROM old)
            "#,
            user_id,
        )
        .fetch_optional(tx.as_mut())
        .await?;
        if let Some(id) = old_id {
            info!("old image id: {:?}", id);
            sqlx::query!("DELETE FROM files WHERE id = $1", id,)
                .execute(tx.as_mut())
                .await?;
        }
        // add new image to files table
        let new_id = sqlx::query_scalar!(
            r#"INSERT INTO files (id, old_file_name, new_file_name, file_type, size_bytes, extension)
                     VALUES (gen_random_uuid(),$1,$2,$3,$4,$5)
                     RETURNING id"#,
            old_name,
            new_name,
            image_type,
            file_size,
            extension
        )
        .fetch_one(tx.as_mut())
        .await?;
        // link image to user
        let result = sqlx::query!(
            "UPDATE users SET image_id = $1 WHERE id = $2",
            new_id,
            user_id
        )
        .execute(tx.as_mut())
        .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        tx.commit().await?;
        Ok(())
    }
    async fn get_user_image(&self, user_id: &str) -> Result<Option<File>, sqlx::Error> {
        sqlx::query_as!(
            File,
            r#"
            SELECT * from files WHERE Id = (SELECT image_id FROM users WHERE id = $1)
        "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_user_profile(&self, user_id: &str) -> Result<UserProfile, sqlx::Error> {
        let user = sqlx::query_as!(
            UserProfile,
            r#"
                SELECT u.id,f.new_file_name || '.' || f.extension AS profile_image_name
                FROM users u
                LEFT JOIN files f ON f.id = u.image_id
                WHERE u.id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if user.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(user.unwrap())
    }
}

#[cfg(test)]
pub mod mocks {
    use super::*;
    use mockall::mock;

    mock! {
        pub UserRepo {}

        #[async_trait]
        impl UserRepoTrait for UserRepo {
            async fn exists_verified(&self, student_id: &str) -> Result<bool, sqlx::Error>;
            async fn get_user_by_id(&self, student_id: &str) -> Result<Option<User>, sqlx::Error>;
            async fn update_user_image(
                &self,
                user_id: &str,
                file_size: i64,
                image_type: &str,
                old_name: &str,
                new_name: &str,
                extension: &str,
            ) -> Result<(), sqlx::Error>;
            async fn get_user_image(&self, user_id: &str) -> Result<Option<File>, sqlx::Error>;
            async fn get_user_profile(&self, user_id: &str) -> Result<UserProfile, sqlx::Error>;
        }
    }
}
