use std::sync::Arc;

use futures_util::TryFutureExt;
use tracing::error;

use crate::{
    db::user_repo::UserRepoTrait,
    dtos::{auth::validate_student_id, user::UserProfileView},
    errors::ErrorMessage,
    utils::{
        file_storage::FileStorageTrait,
        images::{DEFAULT_MAX_IMAGE_SIZE, ValidatedImage},
    },
};

#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<dyn UserRepoTrait>,
    file_storage: Arc<dyn FileStorageTrait>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepoTrait>, file_storage: Arc<dyn FileStorageTrait>) -> Self {
        Self {
            user_repo,
            file_storage,
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
        let disk_filename = format!("{}.{}", new_stored_name, validated_img.format().extension());

        //write new file into storage
        self.file_storage
            .write(disk_filename.as_str(), validated_img.bytes())
            .await
            .map_err(|_| ErrorMessage::ServerError)?;

        //retrieve current image
        let current_image = match self.user_repo.get_user_image(user_id.as_str()).await {
            Ok(img) => img,
            Err(e) => {
                error!("Error fetching current image: {}", e);
                let _ = self.file_storage.delete(&disk_filename).await;
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
            let _ = self.file_storage.delete(&disk_filename).await;
            return Err(ErrorMessage::ServerError);
        }

        if let Some(img) = current_image
            && let Err(e) = self.file_storage.delete(&img.get_full_name()).await
        {
            error!("Failed to delete old image: {}", e);
        }

        Ok(())
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::user_repo::mocks::MockUserRepo;
    use crate::dtos::user::UserProfileRowView;
    use crate::models::file::File;
    use crate::utils::file_storage::mocks::MockFileStorage;
    use crate::utils::images::DEFAULT_MAX_IMAGE_SIZE;
    use chrono::Utc;
    use uuid::Uuid;

    fn make_service(repo: MockUserRepo, storage: MockFileStorage) -> UserService {
        UserService::new(Arc::new(repo), Arc::new(storage))
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
