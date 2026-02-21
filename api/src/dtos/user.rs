use actix_multipart::form::{MultipartForm, json::Json as MpJson, tempfile::TempFile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Metadata {
    name: String,
}

#[derive(MultipartForm)]
pub struct UpdateUserProfileImage {
    #[multipart(limit = "5mb")]
    image: TempFile,
    json: MpJson<Metadata>,
}
