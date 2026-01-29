//! Password hashing utilities using Argon2id.
//!
//! This module provides a thin wrapper around the `argon2` crate
//! to ensure consistent parameters, input validation, and error handling.
//!
//! Passwords are never decrypted — only hashed and verified.

use crate::errors::ErrorMessage;
use argon2::{
    Argon2, Params,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

// Upper bound to prevent DoS via extremely large password inputs.
// Argon2 itself is safe with long inputs, but this protects memory usage.
const MAX_PASSWORD_LENGTH: usize = 64;

/// Service responsible for hashing and verifying passwords.
///
/// Wraps an `Argon2` instance to ensure the same algorithm and
/// parameters are used consistently across the application.
pub struct PasswordHasherService {
    argon2: Argon2<'static>,
}
impl PasswordHasherService {
    pub fn new() -> Self {
        let params = Params::default();
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
        Self { argon2 }
    }
    /// Hashes a plaintext password using Argon2id.
    ///
    /// # Errors
    /// - `EmptyPassword` if the password is empty
    /// - `ExceededMaxPasswordLength` if the password exceeds the limit
    /// - `HashingError` if Argon2 fails
    pub fn hash(&self, password: impl Into<String>) -> Result<String, ErrorMessage> {
        let password = password.into();

        if password.is_empty() {
            return Err(ErrorMessage::EmptyPassword);
        }

        if password.len() > MAX_PASSWORD_LENGTH {
            return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
        }

        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| ErrorMessage::HashingError)?
            .to_string();

        Ok(hashed_password)
    }
    /// Verifies a plaintext password against a stored Argon2 hash.
    ///
    /// Returns `Ok(true)` if the password matches, `Ok(false)` otherwise.
    pub fn compare(&self, password: &str, hashed_password: &str) -> Result<bool, ErrorMessage> {
        if password.is_empty() {
            return Err(ErrorMessage::EmptyPassword);
        }
        if password.len() > MAX_PASSWORD_LENGTH {
            return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
        }
        let parsed_hash =
            PasswordHash::new(hashed_password).map_err(|_| ErrorMessage::InvalidHashFormat)?;

        let password_matches = self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true);

        Ok(password_matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_and_verifies_password() {
        let hasher = PasswordHasherService::new();
        let password = "correct horse battery sample";

        let hash = hasher.hash(password).unwrap();
        let is_valid = hasher.compare(password, &hash).unwrap();

        assert!(is_valid);
    }
    #[test]
    fn wrong_password_fails_verification() {
        let hasher = PasswordHasherService::new();
        let hash = hasher.hash("password123").unwrap();

        let is_valid = hasher.compare("wrong_password", &hash).unwrap();
        assert!(!is_valid)
    }
    #[test]
    fn empty_password_is_rejected() {
        let hasher = PasswordHasherService::new();

        let result = hasher.hash("");
        assert!(matches!(result, Err(ErrorMessage::EmptyPassword)));
    }
    #[test]
    fn password_too_long_is_rejected() {
        let hasher = PasswordHasherService::new();

        let password = "a".repeat(MAX_PASSWORD_LENGTH + 1);

        let result = hasher.hash(password);
        assert!(matches!(
            result,
            Err(ErrorMessage::ExceededMaxPasswordLength(_))
        ));
    }
    #[test]
    fn invalid_hash_format_returns_error() {
        let hasher = PasswordHasherService::new();

        let result = hasher.compare("password", "not-a-valid-hash");
        assert!(matches!(result, Err(ErrorMessage::InvalidHashFormat)));
    }
}
