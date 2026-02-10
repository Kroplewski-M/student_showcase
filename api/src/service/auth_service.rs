use tracing::error;
use uuid::Uuid;

use crate::{
    db::users_repo::UsersRepo,
    errors::ErrorMessage,
    utils::{email::EmailService, password::PasswordHasherService},
};

#[derive(Clone)]
pub struct AuthService {
    user_repo: UsersRepo,
    email_service: EmailService,
}

impl AuthService {
    pub fn new(user_repo: UsersRepo, email_service: EmailService) -> Self {
        Self {
            user_repo,
            email_service,
        }
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
        let verification_token = self
            .user_repo
            .create_user_verification(id.as_str())
            .await
            .map_err(|e| {
                error!("Failed creating a user verification token: {:?}", e);
                ErrorMessage::ServerError
            })?;
        self.email_service
            .send_verification_email(id.clone(), verification_token)
            .await
            .map_err(|e| {
                error!("Failed sending email: {:?}", e);
                ErrorMessage::EmailSendingFailed(
                    "Verification email failed to send but account created successfully"
                        .to_string(),
                )
            })?;
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
}
