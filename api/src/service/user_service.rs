use std::{future, sync::Arc};

use actix_multipart::form::tempfile::TempFile;
use futures_util::{TryFutureExt, future::try_join_all};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    db::user_repo::UserRepoTrait,
    dtos::{
        auth::validate_student_id,
        reference::FileInfo,
        user::{
            ProjectForm, ProjectFormData, ProjectUpsertData, UpdateUserInfo, UserFormData,
            UserProfileView,
        },
    },
    errors::ErrorMessage,
    models::file::File,
    service::reference_service::ReferenceService,
    utils::{
        embedding::Embedding,
        file_storage::FileStorageTrait,
        images::{DEFAULT_MAX_IMAGE_SIZE, ValidatedImage},
    },
};

pub static MAX_IMAGES: usize = 5;
#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<dyn UserRepoTrait>,
    user_file_storage: Arc<dyn FileStorageTrait>,
    project_file_storage: Arc<dyn FileStorageTrait>,
    embedding: Arc<Embedding>,
    reference_service: ReferenceService,
}

impl UserService {
    pub fn new(
        user_repo: Arc<dyn UserRepoTrait>,
        user_file_storage: Arc<dyn FileStorageTrait>,
        project_file_storage: Arc<dyn FileStorageTrait>,
        embedding: Arc<Embedding>,
        reference_service: ReferenceService,
    ) -> Self {
        Self {
            user_repo,
            user_file_storage,
            project_file_storage,
            embedding,
            reference_service,
        }
    }
    pub async fn verified_user_exists(&self, user_id: String) -> Result<bool, ErrorMessage> {
        let valid = validate_student_id(&user_id).map_err(|_| false);
        if valid.is_err() {
            return Ok(false);
        }
        self.user_repo
            .exists_verified(&user_id)
            .await
            .map_err(|_| ErrorMessage::ServerError)
    }
    pub async fn update_user_image(
        &self,
        user_id: String,
        image: Vec<u8>,
        image_name: String,
    ) -> Result<(), ErrorMessage> {
        let validated_img = ValidatedImage::from_bytes(image_name, image, DEFAULT_MAX_IMAGE_SIZE)?;

        let new_stored_name = validated_img.generate_new_filename();
        let disk_filename = validated_img.full_name(&new_stored_name);

        //write new file into storage
        self.user_file_storage
            .write(disk_filename.as_str(), validated_img.bytes())
            .await
            .map_err(|_| ErrorMessage::ServerError)?;

        //retrieve current image
        let current_image = match self.user_repo.get_user_image(user_id.as_str()).await {
            Ok(img) => img,
            Err(e) => {
                error!("Error fetching current image: {}", e);
                let _ = self.user_file_storage.delete(&disk_filename).await;
                return Err(ErrorMessage::ServerError);
            }
        };

        if let Err(e) = self
            .user_repo
            .update_user_image(
                user_id.as_str(),
                validated_img.len(),
                validated_img.format().mime_type(),
                &validated_img.old_name(),
                &new_stored_name,
                validated_img.format().extension(),
            )
            .await
        {
            error!("Error updating user image: {}", e);
            // Compensate: remove the file we just wrote
            let _ = self.user_file_storage.delete(&disk_filename).await;
            return Err(ErrorMessage::ServerError);
        }

        if let Some(img) = current_image
            && let Err(e) = self.user_file_storage.delete(&img.get_full_name()).await
        {
            error!("Failed to delete old image: {}", e);
        }

        Ok(())
    }
    pub async fn get_user_form_data(&self, user_id: String) -> Result<UserFormData, ErrorMessage> {
        let valid = validate_student_id(&user_id).map_err(|_| false);
        if valid.is_err() {
            return Err(ErrorMessage::UserNoLongerExists);
        }
        self.user_repo
            .get_user_form_data(&user_id)
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ErrorMessage::UserNoLongerExists,
                e => {
                    error!("error fetching user form data: {}", e);
                    ErrorMessage::ServerError
                }
            })
            .await
    }
    pub async fn get_user_profile(&self, user_id: String) -> Result<UserProfileView, ErrorMessage> {
        let valid = validate_student_id(&user_id).map_err(|_| false);
        if valid.is_err() {
            return Err(ErrorMessage::UserNoLongerExists);
        }
        self.user_repo
            .get_user_profile(&user_id)
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ErrorMessage::UserNoLongerExists,
                e => {
                    error!("error fetching user profile: {}", e);
                    ErrorMessage::ServerError
                }
            })
            .await
    }
    pub async fn update_user(
        &self,
        user_id: String,
        data: UpdateUserInfo,
    ) -> Result<(), ErrorMessage> {
        let courses = self.reference_service.get_courses().await?;
        let tools = self.reference_service.get_tools().await?;
        let selected_course = data
            .selected_course
            .and_then(|id| courses.iter().find(|c| c.id == id))
            .map(|c| c.name.as_str());
        let tool_names: Vec<String> = data
            .selected_tools
            .iter()
            .filter_map(|id| tools.iter().find(|t| t.id == *id))
            .map(|t| t.name.clone())
            .collect();

        let embed_doc = data.to_embedding_document(selected_course, &tool_names);
        let vector = pgvector::Vector::from(self.embedding.embed_document(embed_doc).await?);
        self.user_repo
            .update_user(user_id.as_str(), data, vector)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ErrorMessage::UserNoLongerExists,
                _ => ErrorMessage::ServerError,
            })?;
        Ok(())
    }
    pub async fn get_user_project_form_data(
        &self,
        user_id: String,
        project_id: Option<Uuid>,
    ) -> Result<ProjectForm, ErrorMessage> {
        let tools = self.reference_service.get_tools().await?;
        let link_types = self.reference_service.get_link_types().await?;
        let mut data: ProjectFormData = ProjectFormData::default();
        if let Some(proj_id) = project_id {
            data = self
                .user_repo
                .get_user_project_form_data(&user_id, proj_id)
                .await
                .map_err(|e| match e {
                    sqlx::Error::RowNotFound => ErrorMessage::ProjectNotFound,
                    _ => ErrorMessage::ServerError,
                })?;
        }
        Ok(ProjectForm {
            project: data,
            tools_list: tools,
            link_types,
        })
    }
    pub async fn upsert_user_project(
        &self,
        user_id: String,
        data: ProjectUpsertData,
        new_images: Vec<TempFile>,
    ) -> Result<(), ErrorMessage> {
        //max images
        if data.existing_images.len() + new_images.len() > MAX_IMAGES {
            return Err(ErrorMessage::TooManyFiles(MAX_IMAGES));
        }
        //get tools
        let tools = self.reference_service.get_tools().await?;
        let tool_names: Vec<String> = data
            .selected_tools
            .iter()
            .filter_map(|id| tools.iter().find(|t| t.id == *id))
            .map(|t| t.name.clone())
            .collect();
        let project_embedding_document = data.to_embedding_document(&tool_names);
        let embedding = self
            .embedding
            .embed_document(project_embedding_document)
            .await?;

        let vector = pgvector::Vector::from(embedding);
        //get all current project images
        let current_files = if let Some(id) = &data.id {
            self.user_repo
                .get_project_files(id)
                .await
                .map_err(|_| ErrorMessage::ServerError)?
        } else {
            Vec::<File>::new()
        };
        //upload new images to storage
        let validated_images: Vec<ValidatedImage> =
            try_join_all(new_images.into_iter().map(|f| async move {
                let file_name = f.file_name.unwrap_or_default();
                let bytes = tokio::fs::read(f.file.path())
                    .await
                    .map_err(|_| ErrorMessage::ServerError)?;
                ValidatedImage::from_bytes(file_name, bytes, DEFAULT_MAX_IMAGE_SIZE)
            }))
            .await?;

        let mut uploaded_images = Vec::<FileInfo>::with_capacity(validated_images.len());
        for file in validated_images {
            let new_name = file.generate_new_filename();
            let disk_filename = file.full_name(&new_name);
            if self
                .project_file_storage
                .write(disk_filename.as_str(), file.bytes())
                .await
                .is_err()
            {
                for f in &uploaded_images {
                    let name = format!("{}.{}", f.new_name, f.extension);
                    if let Err(e) = self.project_file_storage.delete(&name).await {
                        error!(
                            "Failed to delete uploaded project image during rollback {}: {}",
                            name, e
                        );
                    }
                }
                return Err(ErrorMessage::ServerError);
            }
            uploaded_images.push(FileInfo {
                new_name,
                old_name: file.old_name(),
                length: file.len(),
                file_type: file.format().mime_type().to_string(),
                extension: file.format().extension().to_string(),
            });
        }

        // Disk names of newly uploaded files — needed for rollback if DB fails
        let uploaded_disk_names: Vec<String> = uploaded_images
            .iter()
            .map(|f| format!("{}.{}", f.new_name, f.extension))
            .collect();

        // Current files no longer in existing_images — delete from storage on success
        let stale_files: Vec<String> = current_files
            .iter()
            .filter(|f| !data.existing_images.contains(&f.get_full_name()))
            .map(|f| f.get_full_name())
            .collect();

        let res = self
            .user_repo
            .upsert_project(
                &user_id,
                data.id,
                &data.name,
                &data.description,
                data.live_link,
                data.selected_tools,
                data.links,
                uploaded_images,
                data.existing_images,
                vector,
            )
            .await;

        match res {
            Ok(_) => {
                for name in stale_files {
                    if let Err(e) = self.project_file_storage.delete(&name).await {
                        error!("Failed to delete stale project image {}: {}", name, e);
                    }
                }
            }
            Err(_) => {
                for name in uploaded_disk_names {
                    if let Err(e) = self.project_file_storage.delete(&name).await {
                        error!("Failed to delete uploaded project image {}: {}", name, e);
                    }
                }
                return Err(ErrorMessage::ServerError);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::reference_repo::mocks::MockReferenceRepo;
    use crate::db::user_repo::mocks::MockUserRepo;
    use crate::dtos::user::UserProfileRowView;
    use crate::models::file::File;
    use crate::utils::file_storage::mocks::MockFileStorage;
    use crate::utils::generic::MemoryCache;
    use crate::utils::images::DEFAULT_MAX_IMAGE_SIZE;
    use chrono::Utc;
    use moka::future::Cache;
    use uuid::Uuid;

    fn make_reference_service() -> ReferenceService {
        let mut mock_repo = MockReferenceRepo::new();
        mock_repo.expect_get_courses().returning(|| Ok(vec![]));
        mock_repo.expect_get_tools().returning(|| Ok(vec![]));
        mock_repo.expect_get_link_types().returning(|| Ok(vec![]));
        let cache = MemoryCache::new(Cache::builder().max_capacity(100).build());
        ReferenceService::new(Arc::new(mock_repo), cache)
    }

    fn make_service(repo: MockUserRepo, user_storage: MockFileStorage) -> UserService {
        let embedding = Arc::new(Embedding::new(1).expect("Failed to create embedding"));
        UserService::new(
            Arc::new(repo),
            Arc::new(user_storage),
            Arc::new(MockFileStorage::new()),
            embedding,
            make_reference_service(),
        )
    }

    fn dummy_jpeg() -> Vec<u8> {
        vec![
            0xFF, 0xD8, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
    }

    // ── update_user_image ──

    #[tokio::test]
    async fn update_user_image_invalid_format_returns_error() {
        let repo = MockUserRepo::new();
        let storage = MockFileStorage::new();
        let service = make_service(repo, storage);

        let result = service
            .update_user_image("user1".into(), vec![0u8; 12], "photo.jpg".into())
            .await;

        assert!(matches!(result, Err(ErrorMessage::FileInvalidFormat(_))));
    }

    #[tokio::test]
    async fn update_user_image_too_large_returns_error() {
        let repo = MockUserRepo::new();
        let storage = MockFileStorage::new();
        let service = make_service(repo, storage);
        let large_bytes = vec![0u8; DEFAULT_MAX_IMAGE_SIZE + 1];

        let result = service
            .update_user_image("user1".into(), large_bytes, "photo.jpg".into())
            .await;

        assert!(matches!(result, Err(ErrorMessage::FileSizeTooBig(_))));
    }

    #[tokio::test]
    async fn update_user_image_success_no_existing_image() {
        let mut repo = MockUserRepo::new();
        let mut storage = MockFileStorage::new();

        storage.expect_write().returning(|_, _| Ok(()));
        repo.expect_get_user_image().returning(|_| Ok(None));
        repo.expect_update_user_image()
            .returning(|_, _, _, _, _, _| Ok(()));

        let service = make_service(repo, storage);
        let result = service
            .update_user_image("user1".into(), dummy_jpeg(), "photo.jpg".into())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_user_image_success_with_existing_image() {
        let mut repo = MockUserRepo::new();
        let mut storage = MockFileStorage::new();

        storage.expect_write().returning(|_, _| Ok(()));
        storage.expect_delete().returning(|_| Ok(()));
        repo.expect_get_user_image().returning(|_| {
            Ok(Some(File {
                id: Uuid::new_v4(),
                old_file_name: "old_photo".to_string(),
                new_file_name: "old_stored_name".to_string(),
                file_type: "image/jpeg".to_string(),
                size_bytes: 100,
                extension: "jpg".to_string(),
                created_at: Utc::now(),
            }))
        });
        repo.expect_update_user_image()
            .returning(|_, _, _, _, _, _| Ok(()));

        let service = make_service(repo, storage);
        let result = service
            .update_user_image("user1".into(), dummy_jpeg(), "photo.jpg".into())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_user_image_get_repo_error_returns_server_error() {
        let mut repo = MockUserRepo::new();
        let mut storage = MockFileStorage::new();

        storage.expect_write().returning(|_, _| Ok(()));
        storage.expect_delete().returning(|_| Ok(()));
        repo.expect_get_user_image()
            .returning(|_| Err(sqlx::Error::RowNotFound));

        let service = make_service(repo, storage);
        let result = service
            .update_user_image("user1".into(), dummy_jpeg(), "photo.jpg".into())
            .await;

        assert_eq!(result.unwrap_err(), ErrorMessage::ServerError);
    }

    #[tokio::test]
    async fn update_user_image_update_repo_error_returns_server_error() {
        let mut repo = MockUserRepo::new();
        let mut storage = MockFileStorage::new();

        storage.expect_write().returning(|_, _| Ok(()));
        storage.expect_delete().returning(|_| Ok(()));
        repo.expect_get_user_image().returning(|_| Ok(None));
        repo.expect_update_user_image()
            .returning(|_, _, _, _, _, _| Err(sqlx::Error::RowNotFound));

        let service = make_service(repo, storage);
        let result = service
            .update_user_image("user1".into(), dummy_jpeg(), "photo.jpg".into())
            .await;

        assert_eq!(result.unwrap_err(), ErrorMessage::ServerError);
    }
    #[tokio::test]
    async fn get_user_profile_success() {
        let mut repo = MockUserRepo::new();
        let storage = MockFileStorage::new();
        repo.expect_get_user_profile().returning(|_| {
            Ok(UserProfileView {
                base: UserProfileRowView {
                    id: "test-id".to_string(),
                    profile_image_name: None,
                    first_name: None,
                    last_name: None,
                    personal_email: None,
                    course_name: None,
                    description: None,
                },
                certificates: vec![],
                tools: vec![],
                links: vec![],
                projects: vec![],
            })
        });
        let service = make_service(repo, storage);
        let result = service.get_user_profile("2272097".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_user_profile_not_found_maps_to_user_no_longer_exists() {
        let mut repo = MockUserRepo::new();
        let storage = MockFileStorage::new();
        repo.expect_get_user_profile()
            .returning(|_| Err(sqlx::Error::RowNotFound));
        let service = make_service(repo, storage);
        let result = service.get_user_profile("2272097".to_string()).await;
        assert_eq!(result.unwrap_err(), ErrorMessage::UserNoLongerExists);
    }

    #[tokio::test]
    async fn get_user_profile_other_error_maps_to_server_error() {
        let mut repo = MockUserRepo::new();
        let storage = MockFileStorage::new();
        repo.expect_get_user_profile()
            .returning(|_| Err(sqlx::Error::PoolTimedOut));
        let service = make_service(repo, storage);
        let result = service.get_user_profile("2272097".to_string()).await;
        assert_eq!(result.unwrap_err(), ErrorMessage::ServerError);
    }
}
