use crate::errors::ErrorMessage;
use argon2::{
    Argon2, Params,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

const MAX_PASSWORD_LENGTH: usize = 64;

pub struct PasswordHasherService {
    argon2: Argon2<'static>,
}
impl PasswordHasherService {
    pub fn new() -> Self {
        let params = Params::default();
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
        Self { argon2 }
    }
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
