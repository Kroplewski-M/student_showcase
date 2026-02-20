use serde::{Deserialize, Serialize};

pub mod auth;
pub mod user;

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}
