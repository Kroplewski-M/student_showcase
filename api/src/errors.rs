use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt::{self};

use crate::dtos::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}
impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    HashingError,
    InvalidHashFormat,
    InvalidToken,
    ServerError,
    WrongCredentials,
    UserAlreadyExists,
    UserNoLongerExists,
    TokenNotProvided,
    PermissionDenied,
    EmailSendingFailed(String),
    VerifyTokenDoesNotExist,
    UserNotVerified,
}
impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
impl From<ErrorMessage> for String {
    fn from(value: ErrorMessage) -> Self {
        value.to_string()
    }
}
impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
            ErrorMessage::ExceededMaxPasswordLength(length) => {
                format!("Exceeded max password length of {}", length)
            }
            ErrorMessage::HashingError => "Error while hashing password".to_string(),
            ErrorMessage::InvalidHashFormat => "Invalid password hash format".to_string(),
            ErrorMessage::InvalidToken => "Authentication token is invalid or expired".to_string(),
            ErrorMessage::ServerError => "Server error. Please try again later.".to_string(),
            ErrorMessage::WrongCredentials => "Email or password is incorrect".to_string(),
            ErrorMessage::UserAlreadyExists => {
                "A user with this student id already exists".to_string()
            }
            ErrorMessage::UserNoLongerExists => {
                "User belonging to this token no longer exists".to_string()
            }
            ErrorMessage::TokenNotProvided => {
                "You are not logged in, please provide a token".to_string()
            }
            ErrorMessage::PermissionDenied => {
                "You do not have permission to perform this action".to_string()
            }
            ErrorMessage::EmailSendingFailed(error) => {
                format!("Error occured while sending an email: {}", error)
            }
            ErrorMessage::VerifyTokenDoesNotExist => {
                "The token provided does not exist".to_string()
            }
            ErrorMessage::UserNotVerified => {
                "User has not verified their account, please check your email".to_string()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: u16,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: u16) -> Self {
        Self {
            message: message.into(),
            status,
        }
    }
    pub fn server_error(message: impl Into<String>) -> Self {
        Self::new(message, 500)
    }
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(message, 400)
    }
    pub fn unique_constraint_voilation(message: impl Into<String>) -> Self {
        Self::new(message, 409)
    }
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(message, 401)
    }
    pub fn into_http_response(self) -> HttpResponse {
        match self.status {
            400 => HttpResponse::BadRequest().json(Response {
                status: "fail",
                message: self.message,
            }),
            401 => HttpResponse::Unauthorized().json(Response {
                status: "fail",
                message: self.message,
            }),
            409 => HttpResponse::Conflict().json(Response {
                status: "fail",
                message: self.message,
            }),
            500 => HttpResponse::InternalServerError().json(Response {
                status: "fail",
                message: self.message,
            }),
            _ => {
                eprint!(
                    "Warning: Missing pattern match. Cerverted status code {} for 500.",
                    self.status
                );
                HttpResponse::InternalServerError().json(Response {
                    status: "fail",
                    message: ErrorMessage::ServerError.into(),
                })
            }
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "HttpError: message: {}, status: {}",
            self.message, self.status
        )
    }
}

impl std::error::Error for HttpError {}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let cloned = self.clone();
        cloned.into_http_response()
    }
}
