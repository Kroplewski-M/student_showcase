use std::collections::HashMap;

use async_trait::async_trait;
use pgvector::Vector;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    dtos::user::{
        FeaturedProjectCard, ProjImageRow, ProjLinkRow, ProjToolRow, ProjectImageView,
        ProjectProfileView, ProjectProfileViewBase, UpdateUserInfo, UserCardInfo, UserFormData,
        UserLinkView, UserProfileRowView, UserProfileView,
    },
    models::{
        file::File,
        user::{AuthUser, User},
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
    async fn get_auth_user_by_id(&self, student_id: &str) -> Result<Option<AuthUser>, sqlx::Error>;
    async fn update_user_image(
        &self,
        user_id: &str,
        file_size: i64,
        image_type: &str,
        old_name: &str,
        new_name: &str,
        extension: &str,
    ) -> Result<(), sqlx::Error>;
    async fn update_user_cv(
        &self,
        user_id: &str,
        file_size: i64,
        image_type: &str,
        old_name: &str,
        new_name: &str,
        extension: &str,
    ) -> Result<(), sqlx::Error>;
    async fn get_user_current_image(&self, user_id: &str) -> Result<Option<File>, sqlx::Error>;
    async fn get_user_current_cv(&self, user_id: &str) -> Result<Option<File>, sqlx::Error>;
    async fn get_user_profile(&self, user_id: &str) -> Result<UserProfileView, sqlx::Error>;
    async fn get_user_form_data(&self, user_id: &str) -> Result<UserFormData, sqlx::Error>;
    async fn update_user(
        &self,
        user_id: &str,
        data: UpdateUserInfo,
        embedding: Vector,
    ) -> Result<(), sqlx::Error>;
    async fn search_students(&self, embedding: Vector) -> Result<Vec<UserCardInfo>, sqlx::Error>;
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

    async fn get_auth_user_by_id(&self, student_id: &str) -> Result<Option<AuthUser>, sqlx::Error> {
        sqlx::query_as!(
            AuthUser,
            r#"SELECT
            id,
            verified,
            is_admin
            FROM users WHERE id = $1"#,
            student_id
        )
        .fetch_optional(&self.pool)
        .await
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
            password,
            is_admin
            FROM users WHERE id = $1"#,
            student_id
        )
        .fetch_optional(&self.pool)
        .await
    }
    async fn update_user_cv(
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
                SELECT cv_file_id FROM users WHERE id = $1
            )
            UPDATE users
            SET cv_file_id = NULL
            WHERE id = $1
            RETURNING (SELECT cv_file_id FROM old)
            "#,
            user_id,
        )
        .fetch_optional(tx.as_mut())
        .await?;

        if let Some(id) = old_id {
            sqlx::query!("DELETE FROM files WHERE id = $1", id)
                .execute(tx.as_mut())
                .await?;
        }
        // add new cv to files table
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
            "UPDATE users SET cv_file_id = $1 WHERE id = $2",
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
    async fn get_user_current_image(&self, user_id: &str) -> Result<Option<File>, sqlx::Error> {
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
    async fn get_user_current_cv(&self, user_id: &str) -> Result<Option<File>, sqlx::Error> {
        sqlx::query_as!(
            File,
            r#"SELECT * FROM files WHERE Id = (SELECT cv_file_id FROM users WHERE id = $1)"#,
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
                    cv.new_file_name || '.' || cv.extension AS "profile_cv_name?",
                    u.first_name AS "first_name?", 
                    u.last_name AS "last_name?",
                    u.personal_email AS "personal_email?", 
                    c.name AS "course_name?",
                    u.description AS "description?",
                    p.id AS "featured_project_id?",
                    u.suspended AS suspended
                FROM users u
                LEFT JOIN courses c ON u.course_id = c.id
                LEFT JOIN files f ON u.image_id = f.id
                LEFT JOIN files cv ON u.cv_file_id = cv.id
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
            ORDER BY p.featured DESC, p.created_at ASC
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

    async fn search_students(&self, embedding: Vector) -> Result<Vec<UserCardInfo>, sqlx::Error> {
        struct StudentBaseRow {
            user_id: String,
            first_name: Option<String>,
            image_name: Option<String>,
            last_name: Option<String>,
            description: Option<String>,
            course: Option<String>,
            featured_project_id: Option<Uuid>,
            featured_project_name: Option<String>,
            featured_project_description: Option<String>,
        }
        //inner joining on projects as we dont want to return students without any projects
        let bases = sqlx::query_as!(
            StudentBaseRow,
            r#"
            WITH
            search_vec AS (
                SELECT $1::vector AS vec
            ),
            best_project_dist AS (
                SELECT p.user_id, MIN(p.embedding <=> sv.vec) AS min_dist
                FROM projects p
                CROSS JOIN search_vec sv
                WHERE p.embedding IS NOT NULL
                GROUP BY p.user_id
            )
            SELECT
                u.id AS "user_id!",
                u.first_name,
                u.last_name,
                f.new_file_name || '.' || f.extension AS image_name,
                u.description,
                c.name AS "course",
                fp.id AS "featured_project_id?",
                fp.name AS "featured_project_name?",
                fp.description AS "featured_project_description?"
            FROM users u
            CROSS JOIN search_vec sv
            LEFT JOIN courses c ON u.course_id = c.id
            INNER JOIN projects fp ON fp.user_id = u.id AND fp.featured = true
            LEFT JOIN best_project_dist bpd ON bpd.user_id = u.id
            LEFT JOIN files f ON f.id = u.image_id 
            WHERE 
            u.verified = true
            AND u.suspended = false
            AND (
                (u.embedding IS NOT NULL AND u.embedding <=> sv.vec <= 0.7)
                OR bpd.min_dist <= 0.7
            )
            ORDER BY LEAST(
                COALESCE(u.embedding <=> sv.vec, 1.0),
                COALESCE(bpd.min_dist, 1.0)
            ) ASC
            "#,
            embedding as Vector
        )
        .fetch_all(&self.pool)
        .await?;

        if bases.is_empty() {
            return Ok(vec![]);
        }

        let user_ids: Vec<String> = bases.iter().map(|b| b.user_id.clone()).collect();
        let project_ids: Vec<Uuid> = bases.iter().filter_map(|b| b.featured_project_id).collect();

        let all_user_tools = sqlx::query!(
            r#"
            SELECT ut.user_id AS "user_id!", st.name AS "name!"
            FROM user_tools ut
            JOIN software_tools st ON st.id = ut.software_tool_id
            WHERE ut.user_id = ANY($1)
            ORDER BY st.name
            "#,
            &user_ids as &[String]
        )
        .fetch_all(&self.pool)
        .await?;

        let all_project_tools = sqlx::query_as!(
            ProjToolRow,
            r#"
            SELECT pt.project_id AS "project_id!", st.name AS "name!"
            FROM project_tools pt
            JOIN software_tools st ON st.id = pt.tool_id
            WHERE pt.project_id = ANY($1)
            ORDER BY st.name
            "#,
            &project_ids as &[Uuid]
        )
        .fetch_all(&self.pool)
        .await?;

        let all_project_images = sqlx::query_as!(
            ProjImageRow,
            r#"
            SELECT pf.project_id AS "project_id!", f.id AS "file_id!",
                   f.new_file_name || '.' || f.extension AS "file_name!"
            FROM project_files pf
            JOIN files f ON f.id = pf.file_id
            WHERE pf.project_id = ANY($1)
            ORDER BY f.created_at
            "#,
            &project_ids as &[Uuid]
        )
        .fetch_all(&self.pool)
        .await?;

        let mut user_tools_map: HashMap<String, Vec<String>> = HashMap::new();
        for row in all_user_tools {
            user_tools_map
                .entry(row.user_id)
                .or_default()
                .push(row.name);
        }

        let mut project_tools_map: HashMap<Uuid, Vec<String>> = HashMap::new();
        for row in all_project_tools {
            project_tools_map
                .entry(row.project_id)
                .or_default()
                .push(row.name);
        }

        let mut project_images_map: HashMap<Uuid, Vec<String>> = HashMap::new();
        for row in all_project_images {
            project_images_map
                .entry(row.project_id)
                .or_default()
                .push(row.file_name);
        }

        let results = bases
            .into_iter()
            .map(|b| {
                let featured_project = FeaturedProjectCard {
                    name: b.featured_project_name.unwrap_or_default(),
                    description: b.featured_project_description.unwrap_or_default(),
                    tools: b
                        .featured_project_id
                        .and_then(|id| project_tools_map.remove(&id))
                        .unwrap_or_default(),
                    images: b
                        .featured_project_id
                        .and_then(|id| project_images_map.remove(&id))
                        .unwrap_or_default(),
                };
                UserCardInfo {
                    id: b.user_id.clone(),
                    first_name: b.first_name.unwrap_or_default(),
                    last_name: b.last_name.unwrap_or_default(),
                    profile_image: b.image_name,
                    description: b.description.unwrap_or_default(),
                    course: b.course.unwrap_or_default(),
                    tools: user_tools_map.remove(&b.user_id).unwrap_or_default(),
                    featured_project,
                }
            })
            .collect();

        Ok(results)
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
            async fn get_auth_user_by_id(&self, student_id: &str) -> Result<Option<AuthUser>, sqlx::Error>;
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
            async fn get_user_current_image(&self, user_id: &str) -> Result<Option<File>, sqlx::Error>;
            async fn get_user_profile(&self, user_id: &str) -> Result<UserProfileView, sqlx::Error>;
            async fn get_user_form_data(&self, user_id: &str) -> Result<UserFormData, sqlx::Error>;
            async fn update_user(
                &self,
                user_id: &str,
                data: UpdateUserInfo,
                embedding: Vector,
            ) -> Result<(), sqlx::Error>;
           async fn search_students(&self, embedding: Vector) -> Result<Vec<UserCardInfo>, sqlx::Error>;
           async fn get_user_current_cv(&self, user_id: &str) -> Result<Option<File>, sqlx::Error>;
           async fn update_user_cv(
                    &self,
                    user_id: &str,
                    file_size: i64,
                    image_type: &str,
                    old_name: &str,
                    new_name: &str,
                    extension: &str,
          ) -> Result<(), sqlx::Error>;

        }
    }
}
