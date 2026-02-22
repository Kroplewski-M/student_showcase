use std::sync::Arc;

use tracing::error;

use crate::{
    config::Config,
    db::user_repo::UserRepoTrait,
    errors::ErrorMessage,
    utils::{
        file_storage::FileStorageType,
        images::{DEFAULT_MAX_IMAGE_SIZE, ValidatedImage},
    },
};

#[derive(Clone)]
pub struct UserService {
    user_repo: Arc<dyn UserRepoTrait>,
    config: Config,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepoTrait>, config: Config) -> Self {
        Self { user_repo, config }
    }
    pub async fn update_user_image(
        &self,
        user_id: String,
        image: Vec<u8>,
        image_name: String,
    ) -> Result<(), ErrorMessage> {
        let validated_img = ValidatedImage::from_bytes(image_name, image, DEFAULT_MAX_IMAGE_SIZE)?;

        let file_type = FileStorageType::UserImage;
        let new_stored_name = validated_img.generate_new_filename();
        let disk_filename = format!("{}.{}", new_stored_name, validated_img.format().extension());

        //write new file into storage
        file_type
            .write(disk_filename.as_str(), validated_img.bytes())
            .await
            .map_err(|_| ErrorMessage::ServerError)?;

        //retrieve current image
        let current_image = match self.user_repo.get_user_image(user_id.as_str()).await {
            Ok(img) => img,
            Err(e) => {
                error!("Error fetching current image: {}", e);
                let _ = file_type.delete(&disk_filename).await;
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
            let _ = file_type.delete(&disk_filename).await;
            return Err(ErrorMessage::ServerError);
        }

        if let Some(img) = current_image
            && let Err(e) = file_type.delete(&img.get_full_name()).await
        {
            error!("Failed to delete old image: {}", e);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::PostMarkConfig;
    use crate::db::user_repo::mocks::MockUserRepo;
    use crate::models::File;
    use crate::utils::images::DEFAULT_MAX_IMAGE_SIZE;
    use chrono::Utc;
    use uuid::Uuid;

    fn test_config() -> Config {
        Config {
            database_url: String::new(),
            jwt_secret: "test_secret".to_string(),
            jwt_max_age_mins: 60,
            port: 8080,
            post_mark_config: PostMarkConfig {
                mail_from_email: String::new(),
                server_token: String::new(),
            },
            auth_cookie_name: "token".to_string(),
            base_url: "http://localhost:3000".to_string(),
            is_prod: false,
        }
    }

    fn make_service(repo: MockUserRepo) -> UserService {
        UserService::new(Arc::new(repo), test_config())
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
        let service = make_service(repo);

        let result = service
            .update_user_image("user1".into(), vec![0u8; 12], "photo.jpg".into())
            .await;

        assert!(matches!(result, Err(ErrorMessage::FileInvalidFormat(_))));
    }

    #[tokio::test]
    async fn update_user_image_too_large_returns_error() {
        let repo = MockUserRepo::new();
        let service = make_service(repo);
        let large_bytes = vec![0u8; DEFAULT_MAX_IMAGE_SIZE + 1];

        let result = service
            .update_user_image("user1".into(), large_bytes, "photo.jpg".into())
            .await;

        assert!(matches!(result, Err(ErrorMessage::FileSizeTooBig(_))));
    }

    #[tokio::test]
    async fn update_user_image_success_no_existing_image() {
        let mut repo = MockUserRepo::new();

        repo.expect_get_user_image().returning(|_| Ok(None));
        repo.expect_update_user_image()
            .returning(|_, _, _, _, _, _| Ok(()));

        let service = make_service(repo);
        let result = service
            .update_user_image("user1".into(), dummy_jpeg(), "photo.jpg".into())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_user_image_success_with_existing_image() {
        let mut repo = MockUserRepo::new();

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

        let service = make_service(repo);
        let result = service
            .update_user_image("user1".into(), dummy_jpeg(), "photo.jpg".into())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_user_image_get_repo_error_returns_server_error() {
        let mut repo = MockUserRepo::new();

        repo.expect_get_user_image()
            .returning(|_| Err(sqlx::Error::RowNotFound));

        let service = make_service(repo);
        let result = service
            .update_user_image("user1".into(), dummy_jpeg(), "photo.jpg".into())
            .await;

        assert_eq!(result.unwrap_err(), ErrorMessage::ServerError);
    }

    #[tokio::test]
    async fn update_user_image_update_repo_error_returns_server_error() {
        let mut repo = MockUserRepo::new();

        repo.expect_get_user_image().returning(|_| Ok(None));
        repo.expect_update_user_image()
            .returning(|_, _, _, _, _, _| Err(sqlx::Error::RowNotFound));

        let service = make_service(repo);
        let result = service
            .update_user_image("user1".into(), dummy_jpeg(), "photo.jpg".into())
            .await;

        assert_eq!(result.unwrap_err(), ErrorMessage::ServerError);
    }
}
