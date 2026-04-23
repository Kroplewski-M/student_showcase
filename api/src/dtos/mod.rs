use serde::{Deserialize, Serialize};

pub mod admin;
pub mod auth;
pub mod reference;
pub mod user;

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}
