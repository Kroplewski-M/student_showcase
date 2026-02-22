use async_trait::async_trait;
use tokio::fs;

use crate::errors::ErrorMessage;
use std::path::PathBuf;

#[cfg(not(test))]
const BASE_PATH: &str = "/srv/uploads";

#[cfg(test)]
const BASE_PATH: &str = "./test_uploads";

#[async_trait]
pub trait FileStorageTrait: Send + Sync {
    async fn write(&self, name: &str, data: &[u8]) -> Result<(), ErrorMessage>;
    async fn delete(&self, name: &str) -> Result<(), ErrorMessage>;
}

pub enum FileStorageType {
    UserImage,
    ProjectImage,
}

impl FileStorageType {
    fn directory_path(&self) -> PathBuf {
        let sub = match self {
            Self::UserImage => "user_images",
            Self::ProjectImage => "project_images",
        };
        PathBuf::from(BASE_PATH).join(sub)
    }
}

#[async_trait]
impl FileStorageTrait for FileStorageType {
    async fn write(&self, name: &str, data: &[u8]) -> Result<(), ErrorMessage> {
        if name.is_empty()
            || name.contains("..")
            || name.contains('/')
            || name.contains('\\')
            || name.contains('\0')
        {
            return Err(ErrorMessage::FileInvalidName);
        }
        let dir = self.directory_path();

        fs::create_dir_all(&dir)
            .await
            .map_err(|_| ErrorMessage::ServerError)?;
        let path = dir.join(name);

        fs::write(&path, data)
            .await
            .map_err(|_| ErrorMessage::ServerError)?;
        Ok(())
    }

    async fn delete(&self, name: &str) -> Result<(), ErrorMessage> {
        if name.is_empty()
            || name.contains("..")
            || name.contains('/')
            || name.contains('\\')
            || name.contains('\0')
        {
            return Err(ErrorMessage::FileInvalidName);
        }
        let path = self.directory_path().join(name);
        match fs::remove_file(&path).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(_) => Err(ErrorMessage::ServerError),
        }
    }
}

#[cfg(test)]
pub mod mocks {
    use super::*;
    use mockall::mock;

    mock! {
        pub FileStorage {}

        #[async_trait]
        impl FileStorageTrait for FileStorage {
            async fn write(&self, name: &str, data: &[u8]) -> Result<(), ErrorMessage>;
            async fn delete(&self, name: &str) -> Result<(), ErrorMessage>;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    fn test_storage() -> FileStorageType {
        FileStorageType::UserImage
    }

    #[tokio::test]
    async fn write_creates_file_successfully() {
        let storage = test_storage();
        let file_name = "test_file.txt";
        let data = b"hello world";

        let result = storage.write(file_name, data).await;
        assert!(result.is_ok());

        let path = storage.directory_path().join(file_name);
        let exists = fs::try_exists(&path).await.unwrap();
        assert!(exists);

        // cleanup
        storage.delete(file_name).await.unwrap();
    }

    #[tokio::test]
    async fn delete_removes_existing_file() {
        let storage = test_storage();
        let file_name = "delete_me.txt";
        let data = b"delete content";

        storage.write(file_name, data).await.unwrap();

        let result = storage.delete(file_name).await;
        assert!(result.is_ok());

        let path = storage.directory_path().join(file_name);
        let exists = fs::try_exists(&path).await.unwrap();
        assert!(!exists);
    }

    #[tokio::test]
    async fn delete_non_existing_file_returns_ok() {
        let storage = test_storage();
        let result = storage.delete("does_not_exist.txt").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn write_rejects_invalid_filename_with_dots() {
        let storage = test_storage();
        let result = storage.write("../evil.txt", b"bad").await;
        assert!(matches!(result, Err(ErrorMessage::FileInvalidName)));
    }

    #[tokio::test]
    async fn write_rejects_invalid_filename_with_slash() {
        let storage = test_storage();
        let result = storage.write("folder/evil.txt", b"bad").await;
        assert!(matches!(result, Err(ErrorMessage::FileInvalidName)));
    }

    #[tokio::test]
    async fn delete_rejects_invalid_filename() {
        let storage = test_storage();
        let result = storage.delete("../evil.txt").await;
        assert!(matches!(result, Err(ErrorMessage::FileInvalidName)));
    }
}
