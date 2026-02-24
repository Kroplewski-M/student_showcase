use crate::errors::ErrorMessage;
use async_trait::async_trait;
use image::{Frame, ImageFormat};
use std::path::PathBuf;
use tokio::fs;

#[cfg(not(test))]
const BASE_PATH: &str = "/srv/uploads";

#[cfg(test)]
const BASE_PATH: &str = "./test_uploads";

#[async_trait]
pub trait FileStorageTrait: Send + Sync {
    async fn write(&self, name: &str, data: &[u8]) -> Result<(), ErrorMessage>;
    async fn delete(&self, name: &str) -> Result<(), ErrorMessage>;
    fn strip_metadata(&self, name: &str, data: &[u8]) -> Result<Vec<u8>, ErrorMessage>;
    fn strip_gif_metadata(&self, data: &[u8]) -> Result<Vec<u8>, ErrorMessage>;
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

        // Strip metadata by re-encoding the image
        let clean_data = self.strip_metadata(name, data)?;

        let dir = self.directory_path();

        fs::create_dir_all(&dir)
            .await
            .map_err(|_| ErrorMessage::ServerError)?;
        let path = dir.join(name);

        fs::write(&path, clean_data)
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

    fn strip_metadata(&self, name: &str, data: &[u8]) -> Result<Vec<u8>, ErrorMessage> {
        let format =
            ImageFormat::from_path(name).map_err(|_| ErrorMessage::FileInvalidFormat(None))?;
        if format == ImageFormat::Gif {
            return self.strip_gif_metadata(data);
        }
        //Decode fully (drops all EXIF, GPS, XMP, IPTC metadata)
        let img = image::load_from_memory_with_format(data, format)
            .map_err(|_| ErrorMessage::FileInvalidFormat(None))?;

        // Re-encode into a clean buffer
        let mut buf = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut buf);
        img.write_to(&mut cursor, format)
            .map_err(|_| ErrorMessage::ServerError)?;
        Ok(buf)
    }
    fn strip_gif_metadata(&self, data: &[u8]) -> Result<Vec<u8>, ErrorMessage> {
        use image::AnimationDecoder;
        use image::codecs::gif::{GifDecoder, GifEncoder, Repeat};

        let decoder = GifDecoder::new(std::io::Cursor::new(data))
            .map_err(|_| ErrorMessage::FileInvalidFormat(None))?;

        let frames: Vec<Frame> = decoder
            .into_frames()
            .collect_frames()
            .map_err(|_| ErrorMessage::FileInvalidFormat(None))?;

        let mut buf = Vec::new();
        {
            let mut encoder = GifEncoder::new(&mut buf);
            encoder
                .set_repeat(Repeat::Infinite)
                .map_err(|_| ErrorMessage::ServerError)?;
            encoder
                .encode_frames(frames.into_iter())
                .map_err(|_| ErrorMessage::ServerError)?;
        }
        Ok(buf)
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
            fn strip_metadata(&self, name: &str, data: &[u8]) -> Result<Vec<u8>, ErrorMessage>;
            fn strip_gif_metadata(&self, data: &[u8]) -> Result<Vec<u8>, ErrorMessage>;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::codecs::gif::{GifEncoder, Repeat};
    use image::{ImageFormat, RgbaImage};
    use tokio::fs;

    fn test_storage() -> FileStorageType {
        FileStorageType::UserImage
    }

    /// Create a minimal valid PNG in memory
    fn create_test_png() -> Vec<u8> {
        let img = RgbaImage::new(1, 1);
        let mut buf = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut buf);
        img.write_to(&mut cursor, ImageFormat::Png).unwrap();
        buf
    }

    /// Create a minimal valid JPEG in memory
    fn create_test_jpeg() -> Vec<u8> {
        let img = RgbaImage::new(1, 1);
        let rgb = image::DynamicImage::ImageRgba8(img).to_rgb8();
        let mut buf = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut buf);
        image::DynamicImage::ImageRgb8(rgb)
            .write_to(&mut cursor, ImageFormat::Jpeg)
            .unwrap();
        buf
    }

    /// Create a minimal valid animated GIF with 2 frames
    fn create_test_gif() -> Vec<u8> {
        use image::{Frame, RgbaImage};

        let frame1 = Frame::new(RgbaImage::new(2, 2));
        let frame2 = Frame::new(RgbaImage::new(2, 2));

        let mut buf = Vec::new();
        {
            let mut encoder = GifEncoder::new(&mut buf);
            encoder.set_repeat(Repeat::Infinite).unwrap();
            encoder.encode_frames(vec![frame1, frame2]).unwrap();
        }
        buf
    }

    // --- write tests ---

    #[tokio::test]
    async fn write_creates_file_successfully() {
        let storage = test_storage();
        let file_name = "test_file.png";
        let data = create_test_png();

        let result = storage.write(file_name, &data).await;
        assert!(result.is_ok());

        let path = storage.directory_path().join(file_name);
        assert!(fs::try_exists(&path).await.unwrap());

        storage.delete(file_name).await.unwrap();
    }

    #[tokio::test]
    async fn write_rejects_empty_filename() {
        let storage = test_storage();
        let result = storage.write("", b"data").await;
        assert!(matches!(result, Err(ErrorMessage::FileInvalidName)));
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
    async fn write_rejects_invalid_filename_with_backslash() {
        let storage = test_storage();
        let result = storage.write("folder\\evil.txt", b"bad").await;
        assert!(matches!(result, Err(ErrorMessage::FileInvalidName)));
    }

    #[tokio::test]
    async fn write_rejects_invalid_filename_with_null() {
        let storage = test_storage();
        let result = storage.write("evil\0.txt", b"bad").await;
        assert!(matches!(result, Err(ErrorMessage::FileInvalidName)));
    }

    // --- delete tests ---

    #[tokio::test]
    async fn delete_removes_existing_file() {
        let storage = test_storage();
        let file_name = "delete_me.png";
        let data = create_test_png();

        storage.write(file_name, &data).await.unwrap();

        let result = storage.delete(file_name).await;
        assert!(result.is_ok());

        let path = storage.directory_path().join(file_name);
        assert!(!fs::try_exists(&path).await.unwrap());
    }

    #[tokio::test]
    async fn delete_non_existing_file_returns_ok() {
        let storage = test_storage();
        let result = storage.delete("does_not_exist.txt").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn delete_rejects_invalid_filename() {
        let storage = test_storage();
        let result = storage.delete("../evil.txt").await;
        assert!(matches!(result, Err(ErrorMessage::FileInvalidName)));
    }

    // --- strip_metadata tests ---

    #[tokio::test]
    async fn strip_metadata_returns_valid_png() {
        let storage = test_storage();
        let data = create_test_png();

        let result = storage.strip_metadata("image.png", &data);
        assert!(result.is_ok());

        // Verify the output is still a valid image
        let clean = result.unwrap();
        assert!(image::load_from_memory_with_format(&clean, ImageFormat::Png).is_ok());
    }

    #[tokio::test]
    async fn strip_metadata_returns_valid_jpeg() {
        let storage = test_storage();
        let data = create_test_jpeg();

        let result = storage.strip_metadata("photo.jpg", &data);
        assert!(result.is_ok());

        let clean = result.unwrap();
        assert!(image::load_from_memory_with_format(&clean, ImageFormat::Jpeg).is_ok());
    }

    #[tokio::test]
    async fn strip_metadata_rejects_invalid_extension() {
        let storage = test_storage();
        let result = storage.strip_metadata("file.xyz", b"not an image");
        assert!(matches!(result, Err(ErrorMessage::FileInvalidFormat(_))));
    }

    #[tokio::test]
    async fn strip_metadata_rejects_corrupt_data() {
        let storage = test_storage();
        let result = storage.strip_metadata("image.png", b"not actually a png");
        assert!(matches!(result, Err(ErrorMessage::FileInvalidFormat(_))));
    }

    #[tokio::test]
    async fn strip_metadata_delegates_gif_to_strip_gif_metadata() {
        let storage = test_storage();
        let data = create_test_gif();

        let result = storage.strip_metadata("animation.gif", &data);
        assert!(result.is_ok());
    }

    // --- strip_gif_metadata tests ---

    #[tokio::test]
    async fn strip_gif_metadata_preserves_animation() {
        use image::AnimationDecoder;
        use image::codecs::gif::GifDecoder;

        let storage = test_storage();
        let data = create_test_gif();

        let result = storage.strip_gif_metadata(&data);
        assert!(result.is_ok());

        let clean = result.unwrap();
        let decoder = GifDecoder::new(std::io::Cursor::new(&clean)).unwrap();
        let frames: Vec<_> = decoder.into_frames().collect_frames().unwrap();
        assert_eq!(frames.len(), 2, "animated GIF should preserve both frames");
    }

    #[tokio::test]
    async fn strip_gif_metadata_rejects_invalid_data() {
        let storage = test_storage();
        let result = storage.strip_gif_metadata(b"not a gif");
        assert!(matches!(result, Err(ErrorMessage::FileInvalidFormat(_))));
    }
}
