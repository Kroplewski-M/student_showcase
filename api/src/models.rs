use actix_multipart::Multipart;
use chrono::prelude::*;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::ErrorMessage, utils::images::DEFAULT_MAX_IMAGE_SIZE};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub personal_email: Option<String>,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct File {
    pub id: Uuid,
    pub old_file_name: String,
    pub new_file_name: String,
    pub file_type: String,
    pub size_bytes: i64,
    pub extension: String,
    pub created_at: DateTime<Utc>,
}
impl File {
    pub fn get_full_name(&self) -> String {
        format!("{}.{}", self.new_file_name, self.extension)
    }
}
pub struct FormFile {
    pub name: String,
    pub bytes: Vec<u8>,
}
impl FormFile {
    pub async fn new_from_form_multi_part(mut form_file: Multipart) -> Result<Self, ErrorMessage> {
        let mut field = form_file
            .next()
            .await
            .ok_or(ErrorMessage::NoFileProvided)?
            .map_err(|_| ErrorMessage::InvalidFileData)?;

        // Extract filename
        let content_disposition = field
            .content_disposition()
            .ok_or(ErrorMessage::InvalidFileData)?;

        let filename = content_disposition
            .get_filename()
            .ok_or(ErrorMessage::InvalidFileData)?
            .to_string();

        // Read file bytes
        let mut bytes = Vec::new();

        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|_| ErrorMessage::InvalidFileData)?;
            if bytes.len() + data.len() > DEFAULT_MAX_IMAGE_SIZE {
                return Err(ErrorMessage::FileSizeTooBig(DEFAULT_MAX_IMAGE_SIZE));
            }
            bytes.extend_from_slice(&data);
        }

        Ok(Self {
            name: filename,
            bytes,
        })
    }
}
