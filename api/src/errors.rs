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
    FileSizeTooBig(usize),
    FileInvalidFormat(Vec<String>),
    FileInvalidName,
    NoFileProvided,
    InvalidFileData,
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
                format!("Error occurred while sending an email: {}", error)
            }
            ErrorMessage::VerifyTokenDoesNotExist => {
                "The token provided does not exist".to_string()
            }
            ErrorMessage::UserNotVerified => {
                "User has not verified their account, please check your email".to_string()
            }
            ErrorMessage::FileSizeTooBig(size) => {
                format!("File size exceeds max: {} MiB", size / (1024 * 1024))
            }
            ErrorMessage::FileInvalidFormat(file_formats) => {
                format!(
                    "Invalid file format. Valid formats: {}",
                    file_formats.join(", ")
                )
            }
            ErrorMessage::FileInvalidName => "Invalid file name".to_string(),
            ErrorMessage::NoFileProvided => "No File Provided".to_string(),
            ErrorMessage::InvalidFileData => "Invalid File Data".to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::ResponseError;
    use actix_web::body::MessageBody;

    // ─── ErrorMessage Display ────────────────────────────────────────

    #[test]
    fn error_message_empty_password_display() {
        let msg = ErrorMessage::EmptyPassword;
        assert_eq!(msg.to_string(), "Password cannot be empty");
    }

    #[test]
    fn error_message_exceeded_max_password_length_display() {
        let msg = ErrorMessage::ExceededMaxPasswordLength(72);
        assert_eq!(msg.to_string(), "Exceeded max password length of 72");
    }

    #[test]
    fn error_message_hashing_error_display() {
        assert_eq!(
            ErrorMessage::HashingError.to_string(),
            "Error while hashing password"
        );
    }

    #[test]
    fn error_message_invalid_hash_format_display() {
        assert_eq!(
            ErrorMessage::InvalidHashFormat.to_string(),
            "Invalid password hash format"
        );
    }

    #[test]
    fn error_message_invalid_token_display() {
        assert_eq!(
            ErrorMessage::InvalidToken.to_string(),
            "Authentication token is invalid or expired"
        );
    }

    #[test]
    fn error_message_server_error_display() {
        assert_eq!(
            ErrorMessage::ServerError.to_string(),
            "Server error. Please try again later."
        );
    }

    #[test]
    fn error_message_wrong_credentials_display() {
        assert_eq!(
            ErrorMessage::WrongCredentials.to_string(),
            "Email or password is incorrect"
        );
    }

    #[test]
    fn error_message_user_already_exists_display() {
        assert_eq!(
            ErrorMessage::UserAlreadyExists.to_string(),
            "A user with this student id already exists"
        );
    }

    #[test]
    fn error_message_user_no_longer_exists_display() {
        assert_eq!(
            ErrorMessage::UserNoLongerExists.to_string(),
            "User belonging to this token no longer exists"
        );
    }

    #[test]
    fn error_message_token_not_provided_display() {
        assert_eq!(
            ErrorMessage::TokenNotProvided.to_string(),
            "You are not logged in, please provide a token"
        );
    }

    #[test]
    fn error_message_permission_denied_display() {
        assert_eq!(
            ErrorMessage::PermissionDenied.to_string(),
            "You do not have permission to perform this action"
        );
    }

    #[test]
    fn error_message_email_sending_failed_display() {
        let msg = ErrorMessage::EmailSendingFailed("SMTP timeout".to_string());
        assert_eq!(
            msg.to_string(),
            "Error occurred while sending an email: SMTP timeout"
        );
    }

    #[test]
    fn error_message_verify_token_does_not_exist_display() {
        assert_eq!(
            ErrorMessage::VerifyTokenDoesNotExist.to_string(),
            "The token provided does not exist"
        );
    }

    #[test]
    fn error_message_user_not_verified_display() {
        assert_eq!(
            ErrorMessage::UserNotVerified.to_string(),
            "User has not verified their account, please check your email"
        );
    }

    #[test]
    fn error_message_file_size_too_big_display() {
        // 10 MiB in bytes
        let size = 10 * 1024 * 1024;
        let msg = ErrorMessage::FileSizeTooBig(size);
        assert_eq!(msg.to_string(), "File size exceeds max: 10 MiB");
    }

    #[test]
    fn error_message_file_size_too_big_rounding() {
        // 5.5 MiB — integer division truncates to 5
        let size = 5 * 1024 * 1024 + 512 * 1024;
        let msg = ErrorMessage::FileSizeTooBig(size);
        assert_eq!(msg.to_string(), "File size exceeds max: 5 MiB");
    }

    #[test]
    fn error_message_file_invalid_format_display() {
        let formats = vec!["png".to_string(), "jpg".to_string(), "webp".to_string()];
        let msg = ErrorMessage::FileInvalidFormat(formats);
        assert_eq!(
            msg.to_string(),
            "Invalid file format. Valid formats: png, jpg, webp"
        );
    }

    #[test]
    fn error_message_file_invalid_format_single() {
        let formats = vec!["pdf".to_string()];
        let msg = ErrorMessage::FileInvalidFormat(formats);
        assert_eq!(msg.to_string(), "Invalid file format. Valid formats: pdf");
    }

    // ─── ErrorMessage → String conversion ────────────────────────────

    #[test]
    fn error_message_into_string() {
        let s: String = ErrorMessage::ServerError.into();
        assert_eq!(s, "Server error. Please try again later.");
    }

    // ─── ErrorMessage PartialEq ──────────────────────────────────────

    #[test]
    fn error_message_equality() {
        assert_eq!(ErrorMessage::InvalidToken, ErrorMessage::InvalidToken);
        assert_ne!(ErrorMessage::InvalidToken, ErrorMessage::ServerError);
    }

    #[test]
    fn error_message_parameterised_equality() {
        assert_eq!(
            ErrorMessage::ExceededMaxPasswordLength(72),
            ErrorMessage::ExceededMaxPasswordLength(72)
        );
        assert_ne!(
            ErrorMessage::ExceededMaxPasswordLength(72),
            ErrorMessage::ExceededMaxPasswordLength(128)
        );
    }

    // ─── ErrorResponse serialisation ─────────────────────────────────

    #[test]
    fn error_response_serialises_to_json() {
        let resp = ErrorResponse {
            status: "fail".to_string(),
            message: "something broke".to_string(),
        };
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["status"], "fail");
        assert_eq!(json["message"], "something broke");
    }

    #[test]
    fn error_response_deserialises_from_json() {
        let json = r#"{"status":"fail","message":"not found"}"#;
        let resp: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, "fail");
        assert_eq!(resp.message, "not found");
    }

    #[test]
    fn error_response_display_is_json() {
        let resp = ErrorResponse {
            status: "fail".to_string(),
            message: "oops".to_string(),
        };
        let displayed = resp.to_string();
        let parsed: serde_json::Value = serde_json::from_str(&displayed).unwrap();
        assert_eq!(parsed["status"], "fail");
        assert_eq!(parsed["message"], "oops");
    }

    // ─── HttpError constructors ──────────────────────────────────────

    #[test]
    fn http_error_new() {
        let err = HttpError::new("test error", 418);
        assert_eq!(err.message, "test error");
        assert_eq!(err.status, 418);
    }

    #[test]
    fn http_error_server_error() {
        let err = HttpError::server_error("db crashed");
        assert_eq!(err.status, 500);
        assert_eq!(err.message, "db crashed");
    }

    #[test]
    fn http_error_bad_request() {
        let err = HttpError::bad_request("missing field");
        assert_eq!(err.status, 400);
        assert_eq!(err.message, "missing field");
    }

    #[test]
    fn http_error_unique_constraint_violation() {
        let err = HttpError::unique_constraint_voilation("duplicate email");
        assert_eq!(err.status, 409);
        assert_eq!(err.message, "duplicate email");
    }

    #[test]
    fn http_error_unauthorized() {
        let err = HttpError::unauthorized("bad token");
        assert_eq!(err.status, 401);
        assert_eq!(err.message, "bad token");
    }

    // ─── HttpError::into_http_response status codes ──────────────────

    #[test]
    fn http_error_response_400() {
        let resp = HttpError::bad_request("invalid").into_http_response();
        assert_eq!(resp.status(), 400);
    }

    #[test]
    fn http_error_response_401() {
        let resp = HttpError::unauthorized("denied").into_http_response();
        assert_eq!(resp.status(), 401);
    }

    #[test]
    fn http_error_response_409() {
        let resp = HttpError::unique_constraint_voilation("conflict").into_http_response();
        assert_eq!(resp.status(), 409);
    }

    #[test]
    fn http_error_response_500() {
        let resp = HttpError::server_error("boom").into_http_response();
        assert_eq!(resp.status(), 500);
    }

    #[test]
    fn http_error_response_unknown_status_falls_back_to_500() {
        let err = HttpError::new("weird", 499);
        let resp = err.into_http_response();
        assert_eq!(resp.status(), 500);
    }

    // ─── HttpError response body validation ──────────────────────────

    fn extract_body_json(resp: HttpResponse) -> serde_json::Value {
        let body_bytes = resp.into_body().try_into_bytes().unwrap();
        serde_json::from_slice(&body_bytes).unwrap()
    }

    #[test]
    fn http_error_response_body_contains_message() {
        let resp = HttpError::bad_request("field missing").into_http_response();
        let json = extract_body_json(resp);
        assert_eq!(json["status"], "fail");
        assert_eq!(json["message"], "field missing");
    }

    #[test]
    fn http_error_response_500_body() {
        let resp = HttpError::server_error("internal").into_http_response();
        let json = extract_body_json(resp);
        assert_eq!(json["status"], "fail");
        assert_eq!(json["message"], "internal");
    }

    #[test]
    fn http_error_unknown_status_body_uses_generic_message() {
        let err = HttpError::new("custom msg", 418);
        let resp = err.into_http_response();
        let json = extract_body_json(resp);
        assert_eq!(json["status"], "fail");
        // Falls back to the generic ServerError message
        assert_eq!(json["message"], "Server error. Please try again later.");
    }

    // ─── HttpError Display & Error trait ─────────────────────────────

    #[test]
    fn http_error_display() {
        let err = HttpError::new("test", 400);
        let displayed = format!("{}", err);
        assert!(displayed.contains("test"));
        assert!(displayed.contains("400"));
    }

    #[test]
    fn http_error_implements_std_error() {
        let err = HttpError::new("test", 500);
        let std_err: &dyn std::error::Error = &err;
        assert!(std_err.to_string().contains("test"));
    }

    // ─── ResponseError trait (Actix integration) ─────────────────────

    #[test]
    fn http_error_response_error_trait_400() {
        let err = HttpError::bad_request("bad");
        let resp = err.error_response();
        assert_eq!(resp.status(), 400);
    }

    #[test]
    fn http_error_response_error_trait_500() {
        let err = HttpError::server_error("internal");
        let resp = err.error_response();
        assert_eq!(resp.status(), 500);
    }

    // ─── Edge cases ──────────────────────────────────────────────────

    #[test]
    fn http_error_accepts_string_message() {
        let msg = String::from("owned string");
        let err = HttpError::new(msg, 400);
        assert_eq!(err.message, "owned string");
    }

    #[test]
    fn http_error_accepts_error_message_enum() {
        let err = HttpError::bad_request(ErrorMessage::EmptyPassword);
        assert_eq!(err.message, "Password cannot be empty");
    }

    #[test]
    fn http_error_clone() {
        let err = HttpError::new("cloneable", 400);
        let cloned = err.clone();
        assert_eq!(err.message, cloned.message);
        assert_eq!(err.status, cloned.status);
    }

    #[test]
    fn error_message_email_failed_with_empty_string() {
        let msg = ErrorMessage::EmailSendingFailed(String::new());
        assert_eq!(msg.to_string(), "Error occurred while sending an email: ");
    }

    #[test]
    fn error_message_file_invalid_format_empty_vec() {
        let msg = ErrorMessage::FileInvalidFormat(vec![]);
        assert_eq!(msg.to_string(), "Invalid file format. Valid formats: ");
    }

    #[test]
    fn error_message_file_size_zero() {
        let msg = ErrorMessage::FileSizeTooBig(0);
        assert_eq!(msg.to_string(), "File size exceeds max: 0 MiB");
    }

    #[test]
    fn error_message_exceeded_password_length_zero() {
        let msg = ErrorMessage::ExceededMaxPasswordLength(0);
        assert_eq!(msg.to_string(), "Exceeded max password length of 0");
    }
}
