use std::ops::Deref;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StudentId(pub String);

impl Deref for StudentId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn validate_student_id(id: &str) -> Result<(), validator::ValidationError> {
    let trimmed = id.trim();
    if (trimmed.len() != 7 || trimmed != id) || !trimmed.chars().all(|c| c.is_ascii_digit()) {
        let mut err = validator::ValidationError::new("invalid_student_id");
        err.message = Some("Id must be exactly 7 numbers".into());
        return Err(err);
    }
    Ok(())
}

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct AuthDto {
    #[validate(custom(function = "validate_student_id"))]
    pub id: StudentId,
}
#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct LoginUserDto {
    #[validate(custom(function = "validate_student_id"))]
    pub id: StudentId,
    pub password: String,
}
#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct RegisterUserDto {
    #[validate(custom(function = "validate_student_id"))]
    pub id: StudentId,
    #[validate(length(
        min = 5,
        max = 64,
        message = "Password must be between 5 and 20 characters"
    ))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    #[serde(rename = "passwordConfirmation")]
    pub password_confirmation: String,
}
