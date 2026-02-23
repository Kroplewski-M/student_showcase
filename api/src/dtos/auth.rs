use std::ops::Deref;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StudentId(pub String);

impl Deref for StudentId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn validate_student_id(id: &str) -> Result<(), validator::ValidationError> {
    let trimmed = id.trim();
    if (trimmed.len() != 7 || trimmed != id) || !trimmed.chars().all(|c| c.is_ascii_digit()) {
        let mut err = validator::ValidationError::new("invalid_student_id");
        err.message = Some("Id must be exactly 7 numbers".into());
        return Err(err);
    }
    Ok(())
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
        max = 20,
        message = "Password must be between 5 and 20 characters"
    ))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    #[serde(rename = "passwordConfirmation")]
    pub password_confirmation: String,
}
#[derive(Debug, Deserialize, Clone, Default, Validate)]
pub struct GetResetPasswordDto {
    #[validate(custom(function = "validate_student_id"))]
    pub id: StudentId,
}
#[derive(Debug, Deserialize, Clone, Default, Validate)]
pub struct ResetPasswordDto {
    pub token: Uuid,
    #[validate(length(
        min = 5,
        max = 20,
        message = "Password must be between 5 and 20 characters"
    ))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    #[serde(rename = "passwordConfirmation")]
    pub password_confirmation: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    // ── StudentId ──

    #[test]
    fn student_id_deref_returns_inner_string() {
        let id = StudentId("1234567".to_string());
        let s: &str = &id;
        assert_eq!(s, "1234567");
    }

    #[test]
    fn student_id_default_is_empty() {
        let id = StudentId::default();
        assert_eq!(&*id, "");
    }

    // ── validate_student_id ──

    #[test]
    fn valid_seven_digit_id_passes() {
        assert!(validate_student_id("1234567").is_ok());
    }

    #[test]
    fn all_zeros_passes() {
        assert!(validate_student_id("0000000").is_ok());
    }

    #[test]
    fn too_short_id_fails() {
        let err = validate_student_id("123456").unwrap_err();
        assert_eq!(err.code, "invalid_student_id");
    }

    #[test]
    fn too_long_id_fails() {
        assert!(validate_student_id("12345678").is_err());
    }

    #[test]
    fn id_with_letters_fails() {
        assert!(validate_student_id("123456a").is_err());
    }

    #[test]
    fn id_with_spaces_fails() {
        assert!(validate_student_id("123 567").is_err());
    }

    #[test]
    fn id_with_leading_whitespace_fails() {
        assert!(validate_student_id(" 1234567").is_err());
    }

    #[test]
    fn id_with_trailing_whitespace_fails() {
        assert!(validate_student_id("1234567 ").is_err());
    }

    #[test]
    fn empty_id_fails() {
        assert!(validate_student_id("").is_err());
    }

    #[test]
    fn id_with_special_chars_fails() {
        assert!(validate_student_id("12345-7").is_err());
    }

    // ── LoginUserDto ──

    #[test]
    fn login_dto_valid() {
        let dto = LoginUserDto {
            id: StudentId("1234567".to_string()),
            password: "password".to_string(),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn login_dto_invalid_id_fails() {
        let dto = LoginUserDto {
            id: StudentId("abc".to_string()),
            password: "password".to_string(),
        };
        let errors = dto.validate().unwrap_err();
        assert!(errors.field_errors().contains_key("id"));
    }

    // ── RegisterUserDto ──

    #[test]
    fn register_dto_valid() {
        let dto = RegisterUserDto {
            id: StudentId("1234567".to_string()),
            password: "securepass".to_string(),
            password_confirmation: "securepass".to_string(),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn register_dto_password_too_short_fails() {
        let dto = RegisterUserDto {
            id: StudentId("1234567".to_string()),
            password: "abcd".to_string(),
            password_confirmation: "abcd".to_string(),
        };
        let errors = dto.validate().unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn register_dto_password_too_long_fails() {
        let dto = RegisterUserDto {
            id: StudentId("1234567".to_string()),
            password: "a".repeat(21),
            password_confirmation: "a".repeat(21),
        };
        let errors = dto.validate().unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn register_dto_password_min_boundary_passes() {
        let dto = RegisterUserDto {
            id: StudentId("1234567".to_string()),
            password: "a".repeat(5),
            password_confirmation: "a".repeat(5),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn register_dto_password_max_boundary_passes() {
        let dto = RegisterUserDto {
            id: StudentId("1234567".to_string()),
            password: "a".repeat(20),
            password_confirmation: "a".repeat(20),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn register_dto_passwords_do_not_match_fails() {
        let dto = RegisterUserDto {
            id: StudentId("1234567".to_string()),
            password: "securepass".to_string(),
            password_confirmation: "different".to_string(),
        };
        let errors = dto.validate().unwrap_err();
        assert!(errors.field_errors().contains_key("password_confirmation"));
    }

    #[test]
    fn register_dto_invalid_id_fails() {
        let dto = RegisterUserDto {
            id: StudentId("bad".to_string()),
            password: "securepass".to_string(),
            password_confirmation: "securepass".to_string(),
        };
        let errors = dto.validate().unwrap_err();
        assert!(errors.field_errors().contains_key("id"));
    }

    // ── GetResetPasswordDto ──

    #[test]
    fn get_reset_password_dto_valid() {
        let dto = GetResetPasswordDto {
            id: StudentId("1234567".to_string()),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn get_reset_password_dto_invalid_id_fails() {
        let dto = GetResetPasswordDto {
            id: StudentId("12".to_string()),
        };
        assert!(dto.validate().is_err());
    }

    // ── ResetPasswordDto ──

    #[test]
    fn reset_password_dto_valid() {
        let dto = ResetPasswordDto {
            token: Uuid::new_v4(),
            password: "newpassword".to_string(),
            password_confirmation: "newpassword".to_string(),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn reset_password_dto_short_password_fails() {
        let dto = ResetPasswordDto {
            token: Uuid::new_v4(),
            password: "abcd".to_string(),
            password_confirmation: "abcd".to_string(),
        };
        let errors = dto.validate().unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn reset_password_dto_long_password_fails() {
        let dto = ResetPasswordDto {
            token: Uuid::new_v4(),
            password: "a".repeat(21),
            password_confirmation: "a".repeat(21),
        };
        let errors = dto.validate().unwrap_err();
        assert!(errors.field_errors().contains_key("password"));
    }

    #[test]
    fn reset_password_dto_mismatched_passwords_fails() {
        let dto = ResetPasswordDto {
            token: Uuid::new_v4(),
            password: "newpassword".to_string(),
            password_confirmation: "mismatch".to_string(),
        };
        let errors = dto.validate().unwrap_err();
        assert!(errors.field_errors().contains_key("password_confirmation"));
    }
}
