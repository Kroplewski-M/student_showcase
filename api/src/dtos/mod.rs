use serde::{Deserialize, Serialize};

pub mod auth;

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}
