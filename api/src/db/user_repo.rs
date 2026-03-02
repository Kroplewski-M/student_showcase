use async_trait::async_trait;
use pgvector::Vector;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    dtos::user::{UpdateUserInfo, UserFormData, UserLinkView, UserProfileRowView, UserProfileView},
    models::{file::File, user::User},
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
    async fn get_user_profile(&self, user_id: &str) -> Result<UserProfileView, sqlx::Error>;
    async fn get_user_form_data(&self, user_id: &str) -> Result<UserFormData, sqlx::Error>;
    async fn update_user(
        &self,
        user_id: &str,
        data: UpdateUserInfo,
        embedding: Vector,
    ) -> Result<(), sqlx::Error>;
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

    async fn get_user_profile(&self, user_id: &str) -> Result<UserProfileView, sqlx::Error> {
        let base = sqlx::query_as!(
            UserProfileRowView,
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
            UserLinkView,
            r#"
            SELECT lt.id, lt.name AS "link_type!", 
            ul.url AS "url!",
            ul.name AS "name"
            FROM user_links ul
            JOIN link_types lt ON lt.id = ul.link_type_id
            WHERE ul.user_id = $1
            ORDER BY lt.name, ul.name, ul.url
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(UserProfileView {
            base,
            certificates,
            tools,
            links,
        })
    }

    async fn get_user_form_data(&self, user_id: &str) -> Result<UserFormData, sqlx::Error> {
        struct BaseRow {
            first_name: Option<String>,
            last_name: Option<String>,
            personal_email: Option<String>,
            description: Option<String>,
            course_id: Option<uuid::Uuid>,
        }

        let base = sqlx::query_as!(
            BaseRow,
            r#"
            SELECT
                first_name AS "first_name?",
                last_name AS "last_name?",
                personal_email AS "personal_email?",
                description AS "description?",
                course_id AS "course_id?"
            FROM users
            WHERE id = $1
            AND verified = true
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

        let selected_tools = sqlx::query_scalar!(
            r#"SELECT software_tool_id AS "software_tool_id!" FROM user_tools WHERE user_id = $1"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let certificates = sqlx::query_scalar!(
            r#"SELECT certificate AS "certificate!" FROM user_certificates WHERE user_id = $1 ORDER BY certificate"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let links = sqlx::query_as!(
            UserLinkView,
            r#"
            SELECT lt.id, lt.name AS "link_type!",
            ul.url AS "url!",
            ul.name AS "name"
            FROM user_links ul
            JOIN link_types lt ON lt.id = ul.link_type_id
            WHERE ul.user_id = $1
            ORDER BY lt.name, ul.name, ul.url
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(UserFormData {
            first_name: base.first_name,
            last_name: base.last_name,
            personal_email: base.personal_email,
            description: base.description,
            selected_course: base.course_id,
            selected_tools,
            certificates,
            links,
        })
    }
    async fn update_user(
        &self,
        user_id: &str,
        data: UpdateUserInfo,
        embedding: Vector,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        //update basic user info first
        sqlx::query!(
            r#"
            UPDATE users
            SET first_name = $1,
            last_name = $2,
            personal_email = $3,
            description = $4,
            course_id = $5,
            embedding = $6
            WHERE id = $7
        "#,
            data.first_name,
            data.last_name,
            data.personal_email,
            data.description,
            data.selected_course,
            embedding as Vector,
            user_id,
        )
        .execute(tx.as_mut())
        .await?;

        //reset links
        sqlx::query!("DELETE FROM user_links WHERE user_id = $1", user_id)
            .execute(tx.as_mut())
            .await?;

        if !data.links.is_empty() {
            let link_type_ids: Vec<Uuid> = data.links.iter().map(|l| l.link_type_id).collect();
            let urls: Vec<String> = data.links.iter().map(|l| l.url.clone()).collect();
            let names: Vec<Option<String>> = data.links.iter().map(|l| l.name.clone()).collect();

            sqlx::query!(
                "INSERT INTO user_links (user_id, link_type_id, url, name)
               SELECT $1, * FROM UNNEST(
                   $2::uuid[],
                   $3::text[],
                   $4::text[]
               )",
                user_id,
                &link_type_ids,
                &urls,
                &names as &[Option<String>],
            )
            .execute(tx.as_mut())
            .await?;
        }
        //reset tools
        sqlx::query!("DELETE FROM user_tools WHERE user_id = $1", user_id)
            .execute(tx.as_mut())
            .await?;
        if !data.selected_tools.is_empty() {
            sqlx::query!(
                "INSERT INTO user_tools (user_id, software_tool_id)
                 SELECT $1, * FROM UNNEST($2::uuid[])",
                user_id,
                &data.selected_tools,
            )
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
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
            async fn get_user_profile(&self, user_id: &str) -> Result<UserProfileView, sqlx::Error>;
            async fn get_user_form_data(&self, user_id: &str) -> Result<UserFormData, sqlx::Error>;
            async fn update_user(
                &self,
                user_id: &str,
                data: UpdateUserInfo,
                embedding: Vector,
            ) -> Result<(), sqlx::Error>;
        }
    }
}
