use tracing::error;
use uuid::Uuid;

use crate::{
    config::Config,
    db::users_repo::UsersRepo,
    errors::ErrorMessage,
    utils::{email::EmailService, password::PasswordHasherService, token},
};

#[derive(Clone)]
pub struct AuthService {
    user_repo: UsersRepo,
    email_service: EmailService,
    config: Config,
}

impl AuthService {
    pub fn new(user_repo: UsersRepo, email_service: EmailService, config: Config) -> Self {
        Self {
            user_repo,
            email_service,
            config,
        }
    }
    pub async fn login(
        &self,
        student_id: String,
        password: String,
    ) -> Result<String, ErrorMessage> {
        let result = self
            .user_repo
            .get_user_by_id(student_id.as_str())
            .await
            .map_err(|_| ErrorMessage::ServerError)?;

        let user = result.ok_or(ErrorMessage::WrongCredentials)?;

        let user_password = user.password.ok_or(ErrorMessage::ServerError)?;

        if !user.verified {
            self.create_verification_token_and_send_email(user.id.as_str())
                .await?;
            return Err(ErrorMessage::UserNotVerified);
        }

        let hasher = PasswordHasherService::new();
        let password_matches = hasher
            .compare(&password, user_password.as_str())
            .map_err(|_| ErrorMessage::ServerError)?;

        if password_matches {
            let token = token::create_token(
                &user.id,
                self.config.jwt_secret.as_bytes(),
                self.config.jwt_max_age_mins,
            )
            .map_err(|_| ErrorMessage::ServerError)?;
            return Ok(token);
        }
        Err(ErrorMessage::WrongCredentials)
    }
    pub async fn register(&self, student_id: String, password: String) -> Result<(), ErrorMessage> {
        let hasher = PasswordHasherService::new();
        let hashed_password = hasher.hash(&password).map_err(|e| {
            error!("Password hashing failed: {:?}", e);
            e
        })?;
        let id = self
            .user_repo
            .create_user(&student_id, &hashed_password)
            .await
            .map_err(|e| {
                error!("Failed creating user: {:?}", e);
                match &e {
                    sqlx::Error::Database(db_err) => {
                        if db_err.is_unique_violation() {
                            ErrorMessage::UserAlreadyExists
                        } else {
                            ErrorMessage::ServerError
                        }
                    }
                    _ => ErrorMessage::ServerError,
                }
            })?;
        self.create_verification_token_and_send_email(id.as_str())
            .await?;
        Ok(())
    }
    pub async fn validate_user(&self, token: Uuid) -> Result<(), ErrorMessage> {
        match self.user_repo.validate_user(token).await {
            Ok(_) => Ok(()),
            Err(e) => match &e {
                sqlx::Error::RowNotFound => Err(ErrorMessage::VerifyTokenDoesNotExist),
                _ => Err(ErrorMessage::ServerError),
            },
        }
    }
    async fn create_verification_token_and_send_email(
        &self,
        student_id: &str,
    ) -> Result<(), ErrorMessage> {
        let verification_token = self
            .user_repo
            .create_user_verification(student_id)
            .await
            .map_err(|e| {
                error!("Failed creating a user verification token: {:?}", e);
                ErrorMessage::ServerError
            })?;
        self.email_service
            .send_verification_email(student_id.to_string(), verification_token)
            .await
            .map_err(|e| {
                error!("Failed sending email: {:?}", e);
                ErrorMessage::EmailSendingFailed("Verification email failed to send".to_string())
            })?;
        Ok(())
    }
}
