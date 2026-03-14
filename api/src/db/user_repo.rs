use std::collections::HashMap;

use async_trait::async_trait;
use pgvector::Vector;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    dtos::user::{
        ProjImageRow, ProjLinkRow, ProjToolRow, ProjectFormData, ProjectImageView,
        ProjectProfileView, ProjectProfileViewBase, UpdateUserInfo, UpsertProjectParams,
        UserFormData, UserLinkView, UserProfileRowView, UserProfileView,
    },
    models::{
        file::File,
        user::{ProjectBaseRow, User},
    },
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
    async fn get_user_project_form_data(
        &self,
        user_id: &str,
        project_id: Uuid,
    ) -> Result<ProjectFormData, sqlx::Error>;
    async fn update_user(
        &self,
        user_id: &str,
        data: UpdateUserInfo,
        embedding: Vector,
    ) -> Result<(), sqlx::Error>;
    async fn get_project_files(&self, project_id: &Uuid) -> Result<Vec<File>, sqlx::Error>;
    async fn upsert_project(&self, params: UpsertProjectParams) -> Result<Uuid, sqlx::Error>;
    async fn delete_project(&self, user_id: &str, project_id: Uuid) -> Result<(), sqlx::Error>;
    async fn feature_project(&self, user_id: &str, project_id: Uuid) -> Result<(), sqlx::Error>;
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
        //all user info
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
                    u.description AS "description?",
                    p.id AS "featured_project_id?"
                FROM users u
                LEFT JOIN courses c ON u.course_id = c.id
                LEFT JOIN files f ON u.image_id = f.id
                LEFT JOIN projects p ON p.user_id = u.id AND p.featured = true
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
        // project info
        let project_bases = sqlx::query_as!(
            ProjectProfileViewBase,
            r#"
            SELECT 
                p.id,
                p.name AS "name!",
                p.description AS "description?",
                p.live_link AS "live_link?",
                p.featured_image_id as "featured_img_id?"
            FROM projects p
            WHERE p.user_id = $1
            ORDER BY p.created_at
        "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        let project_ids: Vec<Uuid> = project_bases.iter().map(|p| p.id).collect();
        let all_tools = sqlx::query_as!(
            ProjToolRow,
            r#"
            SELECT pt.project_id AS "project_id!", st.name AS "name!"
            FROM project_tools pt
            JOIN software_tools st ON st.id = pt.tool_id
            WHERE pt.project_id = ANY($1)
            ORDER BY st.name
            "#,
            &project_ids
        )
        .fetch_all(&self.pool)
        .await?;

        let all_images = sqlx::query_as!(
            ProjImageRow,
            r#"
            SELECT pf.project_id AS "project_id!", f.id AS "file_id!",
                   f.new_file_name || '.' || f.extension AS "file_name!"
            FROM project_files pf
            JOIN files f ON f.id = pf.file_id
            WHERE pf.project_id = ANY($1)
            ORDER BY f.created_at
            "#,
            &project_ids
        )
        .fetch_all(&self.pool)
        .await?;

        let all_links = sqlx::query_as!(
            ProjLinkRow,
            r#"
            SELECT pl.project_id AS "project_id!", pl.id AS "id!",
                   lt.name AS "link_type!", pl.url AS "url!", pl.name
            FROM project_links pl
            JOIN link_types lt ON lt.id = pl.link_type_id
            WHERE pl.project_id = ANY($1)
            ORDER BY lt.name, pl.url
            "#,
            &project_ids
        )
        .fetch_all(&self.pool)
        .await?;

        let mut tools_map: HashMap<Uuid, Vec<String>> = HashMap::new();
        for row in all_tools {
            tools_map.entry(row.project_id).or_default().push(row.name);
        }
        let mut images_map: HashMap<Uuid, Vec<ProjectImageView>> = HashMap::new();
        for row in all_images {
            images_map
                .entry(row.project_id)
                .or_default()
                .push(ProjectImageView {
                    file_id: row.file_id,
                    file_name: row.file_name,
                });
        }
        let mut links_map: HashMap<Uuid, Vec<UserLinkView>> = HashMap::new();
        for row in all_links {
            links_map
                .entry(row.project_id)
                .or_default()
                .push(UserLinkView {
                    id: row.id,
                    link_type: row.link_type,
                    url: row.url,
                    name: row.name,
                });
        }

        let projects: Vec<ProjectProfileView> = project_bases
            .into_iter()
            .map(|p| {
                let id = p.id;
                ProjectProfileView {
                    base: p,
                    tools: tools_map.remove(&id).unwrap_or_default(),
                    images: images_map.remove(&id).unwrap_or_default(),
                    links: links_map.remove(&id).unwrap_or_default(),
                }
            })
            .collect();

        Ok(UserProfileView {
            base,
            certificates,
            tools,
            links,
            projects,
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
        let updated_res = sqlx::query!(
            r#"
            UPDATE users
            SET first_name = $1,
            last_name = $2,
            personal_email = $3,
            description = $4,
            course_id = $5,
            embedding = $6,
            updated_at = now()
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
        if updated_res.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

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
                 SELECT DISTINCT $1, * FROM UNNEST($2::uuid[])",
                user_id,
                &data.selected_tools,
            )
            .execute(tx.as_mut())
            .await?;
        }
        //reset certificates
        sqlx::query!("DELETE FROM user_certificates WHERE user_id = $1", user_id)
            .execute(tx.as_mut())
            .await?;
        if !data.certificates.is_empty() {
            sqlx::query!(
                "INSERT INTO user_certificates (user_id, certificate)
                          SELECT $1, * FROM UNNEST($2::text[])",
                user_id,
                &data.certificates
            )
            .execute(tx.as_mut())
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }
    async fn get_user_project_form_data(
        &self,
        user_id: &str,
        project_id: Uuid,
    ) -> Result<ProjectFormData, sqlx::Error> {
        let base = sqlx::query_as!(
            ProjectBaseRow,
            r#"
            SELECT id, name, description, live_link
            FROM projects
            WHERE id = $1
            AND user_id = $2
       "#,
            project_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

        let selected_tools = sqlx::query_scalar!(
            r#"SELECT tool_id AS "tools_id!" FROM project_tools WHERE project_id = $1"#,
            project_id,
        )
        .fetch_all(&self.pool)
        .await?;

        let links = sqlx::query_as!(
            UserLinkView,
            r#"
          SELECT lt.id, lt.name AS "link_type!", pl.url AS "url!", pl.name
          FROM project_links pl
          JOIN link_types lt ON lt.id = pl.link_type_id
          WHERE pl.project_id = $1
          ORDER BY lt.name, pl.url
          "#,
            project_id
        )
        .fetch_all(&self.pool)
        .await?;

        let existing_images = sqlx::query_scalar!(
            r#"
          SELECT f.new_file_name || '.' || f.extension AS "file_name!"
          FROM project_files pf
          JOIN files f ON f.id = pf.file_id
          WHERE pf.project_id = $1
          ORDER BY f.created_at
          "#,
            project_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(ProjectFormData {
            id: Some(base.id),
            name: base.name,
            description: base.description,
            live_link: base.live_link,
            links,
            selected_tools,
            existing_images,
        })
    }

    async fn get_project_files(&self, project_id: &Uuid) -> Result<Vec<File>, sqlx::Error> {
        sqlx::query_as!(
            File,
            r#"SELECT f.* FROM project_files pf
                                 JOIN files f ON pf.file_id = f.id
                                 WHERE pf.project_id = $1"#,
            project_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn upsert_project(&self, params: UpsertProjectParams) -> Result<Uuid, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Insert or update the project record
        let id = if let Some(id) = params.project_id {
            let result = sqlx::query!(
                r#"
                UPDATE projects
                SET name = $1, description = $2, live_link = $3, embedding = $6, updated_at = now()
                WHERE id = $4 AND user_id = $5
                "#,
                params.name,
                params.description,
                params.live_link,
                id,
                params.user_id,
                params.embedding as Vector,
            )
            .execute(tx.as_mut())
            .await?;

            if result.rows_affected() == 0 {
                return Err(sqlx::Error::RowNotFound);
            }
            id
        } else {
            sqlx::query_scalar!(
                r#"
                INSERT INTO projects (id, user_id, name, description, live_link, embedding, featured)
                VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, 
                NOT EXISTS (SELECT 1 FROM projects WHERE user_id = $6)
                )
                RETURNING id
                "#,
                params.user_id,
                params.name,
                params.description,
                params.live_link,
                params.embedding as Vector,
                params.user_id
            )
            .fetch_one(tx.as_mut())
            .await?
        };

        // Reset tools
        sqlx::query!("DELETE FROM project_tools WHERE project_id = $1", id)
            .execute(tx.as_mut())
            .await?;

        if !params.selected_tools.is_empty() {
            sqlx::query!(
                "INSERT INTO project_tools (project_id, tool_id)
                 SELECT DISTINCT $1::uuid, * FROM UNNEST($2::uuid[])",
                id as Uuid,
                &params.selected_tools,
            )
            .execute(tx.as_mut())
            .await?;
        }

        // Reset links
        sqlx::query!("DELETE FROM project_links WHERE project_id = $1", id)
            .execute(tx.as_mut())
            .await?;

        if !params.links.is_empty() {
            let link_type_ids: Vec<Uuid> = params.links.iter().map(|l| l.link_type_id).collect();
            let urls: Vec<String> = params.links.iter().map(|l| l.url.clone()).collect();
            let names: Vec<Option<String>> = params.links.iter().map(|l| l.name.clone()).collect();

            sqlx::query!(
                "INSERT INTO project_links (id, project_id, link_type_id, url, name)
                 SELECT gen_random_uuid(), $1, * FROM UNNEST($2::uuid[], $3::text[], $4::text[])",
                id,
                &link_type_ids,
                &urls,
                &names as &[Option<String>],
            )
            .execute(tx.as_mut())
            .await?;
        }

        // Remove images no longer in existing_images
        if params.existing_images.is_empty() {
            let all_file_ids: Vec<Uuid> = sqlx::query_scalar!(
                r#"SELECT file_id AS "file_id!" FROM project_files WHERE project_id = $1"#,
                id
            )
            .fetch_all(tx.as_mut())
            .await?;

            sqlx::query!("DELETE FROM project_files WHERE project_id = $1", id)
                .execute(tx.as_mut())
                .await?;

            if !all_file_ids.is_empty() {
                sqlx::query!("DELETE FROM files WHERE id = ANY($1)", &all_file_ids)
                    .execute(tx.as_mut())
                    .await?;
            }
        } else {
            let stale_file_ids: Vec<Uuid> = sqlx::query_scalar!(
                r#"
                SELECT pf.file_id AS "file_id!"
                FROM project_files pf
                JOIN files f ON f.id = pf.file_id
                WHERE pf.project_id = $1
                AND f.new_file_name || '.' || f.extension != ALL($2)
                "#,
                id,
                &params.existing_images,
            )
            .fetch_all(tx.as_mut())
            .await?;

            if !stale_file_ids.is_empty() {
                sqlx::query!(
                    "DELETE FROM project_files WHERE file_id = ANY($1)",
                    &stale_file_ids,
                )
                .execute(tx.as_mut())
                .await?;
                sqlx::query!("DELETE FROM files WHERE id = ANY($1)", &stale_file_ids,)
                    .execute(tx.as_mut())
                    .await?;
            }
        }

        // Insert new image files and link to project
        for img in params.new_images {
            let file_id = sqlx::query_scalar!(
                r#"
                INSERT INTO files (id, old_file_name, new_file_name, file_type, size_bytes, extension)
                VALUES (gen_random_uuid(), $1, $2, $3, $4, $5)
                RETURNING id
                "#,
                img.old_name,
                img.new_name,
                img.file_type,
                img.length,
                img.extension,
            )
            .fetch_one(tx.as_mut())
            .await?;

            sqlx::query!(
                "INSERT INTO project_files (project_id, file_id) VALUES ($1, $2)",
                id,
                file_id,
            )
            .execute(tx.as_mut())
            .await?;
        }

        tx.commit().await?;
        Ok(id)
    }
    async fn delete_project(&self, user_id: &str, project_id: Uuid) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // 1. Check if project exists and belongs to user
        let exists: bool = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
            SELECT 1
            FROM projects
            WHERE id = $1
            AND user_id = $2
        )
        "#,
            project_id,
            user_id,
        )
        .fetch_one(tx.as_mut())
        .await?
        .unwrap_or(false);

        if !exists {
            return Err(sqlx::Error::RowNotFound);
        }
        // 2. Remove project tools
        sqlx::query!(
            r#"
                DELETE FROM project_tools
                where project_id = $1
            "#,
            project_id
        )
        .execute(tx.as_mut())
        .await?;
        // 3. Remove project links
        sqlx::query!(
            r#"
                DELETE FROM project_links 
                where project_id = $1
            "#,
            project_id
        )
        .execute(tx.as_mut())
        .await?;
        // 4. Remove project_files (join table)
        let removed_files = sqlx::query_scalar!(
            r#"
            DELETE FROM project_files
            where project_id = $1
            RETURNING file_id
        "#,
            project_id
        )
        .fetch_all(tx.as_mut())
        .await?;
        // 5. Remove files
        sqlx::query!(
            r#"
            DELETE FROM files WHERE id = ANY($1)
        "#,
            &removed_files as &[Uuid],
        )
        .execute(tx.as_mut())
        .await?;
        // 6. Remove project
        let was_featured = sqlx::query_scalar!(
            r#"
            DELETE FROM projects
            WHERE id = $1
            RETURNING featured
        "#,
            project_id
        )
        .fetch_one(tx.as_mut())
        .await?;

        if was_featured {
            sqlx::query!(
                r#"
            UPDATE projects
            SET featured = true
            WHERE id = (
                SELECT id FROM projects
                WHERE user_id = $1
                ORDER BY created_at ASC
                LIMIT 1
            )
            "#,
                user_id
            )
            .execute(tx.as_mut())
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }
    async fn feature_project(&self, user_id: &str, project_id: Uuid) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            r#"
                UPDATE projects
                SET featured = false
                WHERE user_id = $1
                "#,
            user_id
        )
        .execute(tx.as_mut())
        .await?;

        let result = sqlx::query!(
            r#"
                UPDATE projects
                SET featured = true 
                WHERE user_id = $1
                AND id = $2
                "#,
            user_id,
            project_id
        )
        .execute(tx.as_mut())
        .await?;
        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
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
            async fn get_user_project_form_data(
                &self,
                user_id: &str,
                project_id: Uuid,
            ) -> Result<ProjectFormData, sqlx::Error>;
            async fn get_project_files(&self, project_id: &Uuid) -> Result<Vec<File>, sqlx::Error>;
            async fn upsert_project(&self, params: UpsertProjectParams) -> Result<Uuid, sqlx::Error>;
            async fn delete_project(&self, user_id: &str, project_id: Uuid) -> Result<(), sqlx::Error>;
            async fn feature_project(&self, user_id: &str, project_id: Uuid) -> Result<(), sqlx::Error>;
        }
    }
}
