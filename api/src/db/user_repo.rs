use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::models::{
    file::File,
    user::{User, UserLink, UserProfile, UserProfileRow},
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
        let base = sqlx::query_as!(
            UserProfileRow,
            r#"
                SELECT 
                    u.id, 
                    f.new_file_name || '.' || f.extension AS "profile_image_name?",
                    u.first_name AS "first_name?", 
                    u.last_name AS "last_name?",
                    u.personal_email AS "personal_email?", 
                    c.name AS "course_name?",
                    u.description AS "description?"
                FROM users u
                LEFT JOIN courses c ON u.course_id = c.id
                LEFT JOIN files f ON u.image_id = f.id
                WHERE u.id = $1 
                AND u.verified = true
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

        let certificates = sqlx::query_scalar!(
            r#"SELECT certificate AS "certificate!" FROM user_certificates WHERE user_id = $1 ORDER BY certificate"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let tools = sqlx::query_scalar!(
            r#"
            SELECT st.name AS "name!"
            FROM user_tools ut
            JOIN software_tools st ON st.id = ut.software_tool_id
            WHERE ut.user_id = $1
            ORDER BY st.name
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let links = sqlx::query_as!(
            UserLink,
            r#"
            SELECT lt.name AS "link_type!", 
            ul.url AS "url!",
            ul.name AS "name"
            FROM user_links ul
            JOIN link_types lt ON lt.id = ul.link_type_id
            WHERE ul.user_id = $1
            ORDER BY lt.name, ul.name
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(UserProfile {
            base,
            certificates,
            tools,
            links,
        })
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
