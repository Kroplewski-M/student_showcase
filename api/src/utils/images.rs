use std::path::Path;

use crate::errors::ErrorMessage;

/// Default max file size: 5 MiB
pub const DEFAULT_MAX_IMAGE_SIZE: usize = 5 * 1024 * 1024;

/// Validated image type determined from actual file bytes, not client headers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Jpeg,
    Png,
    Webp,
    Gif,
    Avif,
}

impl ImageFormat {
    /// File extension for storage (no dot prefix)
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Webp => "webp",
            Self::Gif => "gif",
            Self::Avif => "avif",
        }
    }
    /// MIME type string
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Webp => "image/webp",
            Self::Gif => "image/gif",
            Self::Avif => "image/avif",
        }
    }
    /// Detect format from magic bytes. Returns None if unrecognised.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 12 {
            return None;
        }

        if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
            Some(Self::Jpeg)
        } else if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
            Some(Self::Png)
        } else if &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
            Some(Self::Webp)
        } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
            Some(Self::Gif)
        } else if &bytes[4..12] == b"ftypavif" {
            Some(Self::Avif)
        } else {
            None
        }
    }
}

/// Validated image ready to be written to disk.
/// Can only be constructed through `ValidatedImage::from_bytes`,
/// which enforces size and format checks.
pub struct ValidatedImage {
    bytes: Vec<u8>,
    format: ImageFormat,
    old_name: String,
}

impl ValidatedImage {
    /// Validate raw bytes for size and format in one step.
    /// On success, returns a ValidatedImage that is guaranteed to be
    /// a recognised image format within the size limit.
    pub fn from_bytes(
        file_name: String,
        bytes: Vec<u8>,
        max_size: usize,
    ) -> Result<Self, ErrorMessage> {
        if bytes.len() > max_size {
            return Err(ErrorMessage::FileSizeTooBig(max_size));
        }
        let valid_extensions: Vec<String> = [
            ImageFormat::Jpeg,
            ImageFormat::Png,
            ImageFormat::Webp,
            ImageFormat::Gif,
            ImageFormat::Avif,
        ]
        .iter()
        .map(|f| f.extension().to_string())
        .collect();

        let format = ImageFormat::from_bytes(&bytes)
            .ok_or(ErrorMessage::FileInvalidFormat(valid_extensions))?;

        let old_name = Path::new(&file_name)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        Ok(Self {
            old_name,
            bytes,
            format,
        })
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn format(&self) -> ImageFormat {
        self.format
    }
    pub fn len(&self) -> i64 {
        self.bytes.len() as i64
    }
    pub fn old_name(&self) -> String {
        self.old_name.clone()
    }
    /// Generate a safe filename with UUID — no user input in the path
    pub fn generate_new_filename(&self) -> String {
        format!("{}", uuid::Uuid::new_v4())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX: usize = 5 * 1024 * 1024;

    fn dummy_jpeg() -> Vec<u8> {
        vec![
            0xFF, 0xD8, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
    }

    fn dummy_png() -> Vec<u8> {
        vec![0x89, 0x50, 0x4E, 0x47, 0, 0, 0, 0, 0, 0, 0, 0]
    }

    fn dummy_webp() -> Vec<u8> {
        let mut v = vec![0; 12];
        v[0..4].copy_from_slice(b"RIFF");
        v[8..12].copy_from_slice(b"WEBP");
        v
    }
    fn dummy_gif() -> Vec<u8> {
        let mut v = b"GIF89a".to_vec();
        v.extend_from_slice(&[0u8; 6]); // pad to 12 bytes
        v
    }

    fn dummy_avif() -> Vec<u8> {
        let mut v = vec![0u8; 12];
        // box-size at [0..4] can be anything; ftyp at [4..8]; avif at [8..12]
        v[4..8].copy_from_slice(b"ftyp");
        v[8..12].copy_from_slice(b"avif");
        v
    }
    #[test]
    fn detects_jpeg() {
        let format = ImageFormat::from_bytes(&dummy_jpeg());
        assert_eq!(format, Some(ImageFormat::Jpeg));
    }

    #[test]
    fn detects_png() {
        let format = ImageFormat::from_bytes(&dummy_png());
        assert_eq!(format, Some(ImageFormat::Png));
    }

    #[test]
    fn detects_webp() {
        let format = ImageFormat::from_bytes(&dummy_webp());
        assert_eq!(format, Some(ImageFormat::Webp));
    }

    #[test]
    fn rejects_invalid_format() {
        let bytes = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let format = ImageFormat::from_bytes(&bytes);
        assert_eq!(format, None);
    }

    #[test]
    fn rejects_small_buffer() {
        let bytes = vec![0xFF, 0xD8];
        let format = ImageFormat::from_bytes(&bytes);
        assert_eq!(format, None);
    }

    #[test]
    fn rejects_file_too_large() {
        let bytes = vec![0u8; MAX + 1];
        let result = ValidatedImage::from_bytes("test.jpg".into(), bytes, MAX);
        assert!(result.is_err());
    }

    #[test]
    fn accepts_valid_image() {
        let bytes = dummy_jpeg();
        let result = ValidatedImage::from_bytes("photo.jpg".into(), bytes, MAX);

        assert!(result.is_ok());

        let img = result.unwrap();
        assert_eq!(img.format(), ImageFormat::Jpeg);
        assert_eq!(img.bytes().len(), 12);
    }
    #[test]
    fn detects_gif() {
        assert_eq!(
            ImageFormat::from_bytes(&dummy_gif()),
            Some(ImageFormat::Gif)
        );
    }

    #[test]
    fn detects_avif() {
        assert_eq!(
            ImageFormat::from_bytes(&dummy_avif()),
            Some(ImageFormat::Avif)
        );
    }
}
