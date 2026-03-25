use async_trait::async_trait;
use pgvector::Vector;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    dtos::user::{ProjectFormData, UpsertProjectParams, UserLinkView},
    models::{file::File, user::ProjectBaseRow},
};

#[derive(Debug, Clone)]
pub struct ProjectRepo {
    pool: Pool<Postgres>,
}
impl ProjectRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
pub trait ProjectRepoTrait: Send + Sync {
    async fn get_project_files(&self, project_id: &Uuid) -> Result<Vec<File>, sqlx::Error>;
    async fn upsert_project(&self, params: UpsertProjectParams) -> Result<Uuid, sqlx::Error>;
    async fn delete_project(&self, user_id: &str, project_id: Uuid) -> Result<(), sqlx::Error>;
    async fn feature_project(&self, user_id: &str, project_id: Uuid) -> Result<(), sqlx::Error>;
    async fn get_user_project_form_data(
        &self,
        user_id: &str,
        project_id: Uuid,
    ) -> Result<ProjectFormData, sqlx::Error>;
}

#[async_trait]
impl ProjectRepoTrait for ProjectRepo {
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
