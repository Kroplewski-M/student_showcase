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
        let current_image = &self
            .user_repo
            .get_user_image(user_id.as_str())
            .await
            .map_err(|_| ErrorMessage::ServerError)?;

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
            error!("Error updaing user image: {}", e);
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
