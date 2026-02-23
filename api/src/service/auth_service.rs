use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

use crate::{
    config::Config,
    db::{auth_repo::AuthRepoTrait, user_repo::UserRepoTrait},
    errors::ErrorMessage,
    utils::{email::EmailServiceTrait, password::PasswordHasherService, token},
};

#[derive(Clone)]
pub struct AuthService {
    auth_repo: Arc<dyn AuthRepoTrait>,
    user_repo: Arc<dyn UserRepoTrait>,
    email_service: Arc<dyn EmailServiceTrait>,
    config: Config,
}

impl AuthService {
    pub fn new(
        auth_repo: Arc<dyn AuthRepoTrait>,
        user_repo: Arc<dyn UserRepoTrait>,
        email_service: Arc<dyn EmailServiceTrait>,
        config: Config,
    ) -> Self {
        Self {
            auth_repo,
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
            .auth_repo
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
        match self.auth_repo.validate_user(token).await {
            Ok(_) => Ok(()),
            Err(e) => match &e {
                sqlx::Error::RowNotFound => Err(ErrorMessage::VerifyTokenDoesNotExist),
                _ => Err(ErrorMessage::ServerError),
            },
        }
    }
    pub async fn create_user_reset_password(&self, student_id: String) -> Result<(), ErrorMessage> {
        let token = self
            .auth_repo
            .create_user_reset_password(student_id.as_str())
            .await
            .map_err(|e| match &e {
                sqlx::Error::RowNotFound => ErrorMessage::UserNoLongerExists,
                _ => ErrorMessage::ServerError,
            })?;

        self.email_service
            .send_reset_password_email(student_id, token)
            .await
            .map_err(|_| ErrorMessage::ServerError)?;
        Ok(())
    }
    pub async fn user_reset_password_exists(&self, token: Uuid) -> Result<bool, ErrorMessage> {
        self.auth_repo
            .user_reset_password_exists(token)
            .await
            .map_err(|_| ErrorMessage::ServerError)
    }
    pub async fn reset_user_password(
        &self,
        token: Uuid,
        password: String,
    ) -> Result<(), ErrorMessage> {
        let hasher = PasswordHasherService::new();
        let hashed_password = hasher.hash(&password)?;
        match self
            .auth_repo
            .update_user_password(token, &hashed_password)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => match &e {
                sqlx::Error::RowNotFound => Err(ErrorMessage::UserNoLongerExists),
                _ => Err(ErrorMessage::ServerError),
            },
        }
    }
    async fn create_verification_token_and_send_email(
        &self,
        student_id: &str,
    ) -> Result<(), ErrorMessage> {
        let verification_token = self
            .auth_repo
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::PostMarkConfig;
    use crate::db::auth_repo::mocks::MockAuthRepo;
    use crate::db::user_repo::mocks::MockUserRepo;
    use crate::models::user::User;
    use crate::utils::email::mocks::MockEmailService;
    use chrono::Utc;

    fn test_config() -> Config {
        Config {
            database_url: String::new(),
            jwt_secret: "test_secret_key_for_testing".to_string(),
            jwt_max_age_mins: 60,
            port: 8080,
            post_mark_config: PostMarkConfig {
                mail_from_email: String::new(),
                server_token: String::new(),
            },
            auth_cookie_name: "token".to_string(),
            base_url: "http://localhost:3000".to_string(),
            is_prod: false,
        }
    }

    fn verified_user(id: &str, password: &str) -> User {
        let hasher = PasswordHasherService::new();
        let hashed = hasher.hash(password).unwrap();
        User {
            id: id.to_string(),
            first_name: None,
            last_name: None,
            personal_email: None,
            verified: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            password: Some(hashed),
        }
    }

    fn unverified_user(id: &str, password: &str) -> User {
        let mut user = verified_user(id, password);
        user.verified = false;
        user
    }

    fn make_service(
        auth_repo: MockAuthRepo,
        user_repo: MockUserRepo,
        email: MockEmailService,
    ) -> AuthService {
        AuthService::new(
            Arc::new(auth_repo),
            Arc::new(user_repo),
            Arc::new(email),
            test_config(),
        )
    }

    // ── login ──

    #[tokio::test]
    async fn login_success_returns_token() {
        let auth_repo = MockAuthRepo::new();
        let mut user_repo = MockUserRepo::new();
        let email = MockEmailService::new();

        let user = verified_user("1234567", "password123");
        user_repo
            .expect_get_user_by_id()
            .returning(move |_| Ok(Some(user.clone())));

        let service = make_service(auth_repo, user_repo, email);
        let result = service.login("1234567".into(), "password123".into()).await;

        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn login_user_not_found_returns_wrong_credentials() {
        let auth_repo = MockAuthRepo::new();
        let mut user_repo = MockUserRepo::new();
        let email = MockEmailService::new();

        user_repo.expect_get_user_by_id().returning(|_| Ok(None));

        let service = make_service(auth_repo, user_repo, email);

        let result = service.login("1234567".into(), "password".into()).await;

        assert_eq!(result.unwrap_err(), ErrorMessage::WrongCredentials);
    }

    #[tokio::test]
    async fn login_wrong_password_returns_wrong_credentials() {
        let auth_repo = MockAuthRepo::new();
        let mut user_repo = MockUserRepo::new();
        let email = MockEmailService::new();

        let user = verified_user("1234567", "correctpass");
        user_repo
            .expect_get_user_by_id()
            .returning(move |_| Ok(Some(user.clone())));

        let service = make_service(auth_repo, user_repo, email);
        let result = service.login("1234567".into(), "wrongpass".into()).await;

        assert_eq!(result.unwrap_err(), ErrorMessage::WrongCredentials);
    }

    #[tokio::test]
    async fn login_unverified_user_resends_verification_email() {
        let mut auth_repo = MockAuthRepo::new();
        let mut user_repo = MockUserRepo::new();
        let mut email = MockEmailService::new();

        let user = unverified_user("1234567", "password123");
        let verification_token = Uuid::new_v4();

        user_repo
            .expect_get_user_by_id()
            .returning(move |_| Ok(Some(user.clone())));
        auth_repo
            .expect_create_user_verification()
            .returning(move |_| Ok(verification_token));
        email
            .expect_send_verification_email()
            .returning(|_, _| Ok(()));

        let service = make_service(auth_repo, user_repo, email);
        let result = service.login("1234567".into(), "password123".into()).await;

        assert_eq!(result.unwrap_err(), ErrorMessage::UserNotVerified);
    }

    // ── register ──

    #[tokio::test]
    async fn register_success() {
        let mut auth_repo = MockAuthRepo::new();
        let user_repo = MockUserRepo::new();
        let mut email = MockEmailService::new();
        let verification_token = Uuid::new_v4();

        auth_repo
            .expect_create_user()
            .returning(|id, _| Ok(id.to_string()));
        auth_repo
            .expect_create_user_verification()
            .returning(move |_| Ok(verification_token));
        email
            .expect_send_verification_email()
            .returning(|_, _| Ok(()));

        let service = make_service(auth_repo, user_repo, email);
        let result = service
            .register("1234567".into(), "password123".into())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn register_duplicate_user_returns_user_already_exists() {
        let mut auth_repo = MockAuthRepo::new();
        let user_repo = MockUserRepo::new();
        let email = MockEmailService::new();

        auth_repo
            .expect_create_user()
            .returning(|_, _| Err(sqlx::Error::Database(Box::new(TestUniqueViolation))));

        let service = make_service(auth_repo, user_repo, email);
        let result = service
            .register("1234567".into(), "password123".into())
            .await;

        assert_eq!(result.unwrap_err(), ErrorMessage::UserAlreadyExists);
    }

    // ── validate_user ──

    #[tokio::test]
    async fn validate_user_success() {
        let mut auth_repo = MockAuthRepo::new();
        let user_repo = MockUserRepo::new();
        let email = MockEmailService::new();
        let token = Uuid::new_v4();

        auth_repo.expect_validate_user().returning(|_| Ok(()));

        let service = make_service(auth_repo, user_repo, email);
        assert!(service.validate_user(token).await.is_ok());
    }

    #[tokio::test]
    async fn validate_user_token_not_found() {
        let mut auth_repo = MockAuthRepo::new();
        let user_repo = MockUserRepo::new();
        let email = MockEmailService::new();
        let token = Uuid::new_v4();

        auth_repo
            .expect_validate_user()
            .returning(|_| Err(sqlx::Error::RowNotFound));

        let service = make_service(auth_repo, user_repo, email);
        let result = service.validate_user(token).await;

        assert_eq!(result.unwrap_err(), ErrorMessage::VerifyTokenDoesNotExist);
    }

    // ── create_user_reset_password ──

    #[tokio::test]
    async fn create_user_reset_password_success() {
        let mut auth_repo = MockAuthRepo::new();
        let user_repo = MockUserRepo::new();
        let mut email = MockEmailService::new();
        let reset_token = Uuid::new_v4();

        auth_repo
            .expect_create_user_reset_password()
            .returning(move |_| Ok(reset_token));
        email
            .expect_send_reset_password_email()
            .returning(|_, _| Ok(()));

        let service = make_service(auth_repo, user_repo, email);
        assert!(
            service
                .create_user_reset_password("1234567".into())
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn create_user_reset_password_user_not_found() {
        let mut auth_repo = MockAuthRepo::new();
        let user_repo = MockUserRepo::new();
        let email = MockEmailService::new();

        auth_repo
            .expect_create_user_reset_password()
            .returning(|_| Err(sqlx::Error::RowNotFound));

        let service = make_service(auth_repo, user_repo, email);
        let result = service.create_user_reset_password("1234567".into()).await;

        assert_eq!(result.unwrap_err(), ErrorMessage::UserNoLongerExists);
    }

    // ── user_reset_password_exists ──

    #[tokio::test]
    async fn user_reset_password_exists_returns_true() {
        let mut auth_repo = MockAuthRepo::new();
        let user_repo = MockUserRepo::new();
        let email = MockEmailService::new();
        let token = Uuid::new_v4();

        auth_repo
            .expect_user_reset_password_exists()
            .returning(|_| Ok(true));

        let service = make_service(auth_repo, user_repo, email);
        assert!(service.user_reset_password_exists(token).await.unwrap());
    }

    #[tokio::test]
    async fn user_reset_password_exists_returns_false() {
        let mut auth_repo = MockAuthRepo::new();
        let user_repo = MockUserRepo::new();
        let email = MockEmailService::new();
        let token = Uuid::new_v4();

        auth_repo
            .expect_user_reset_password_exists()
            .returning(|_| Ok(false));

        let service = make_service(auth_repo, user_repo, email);
        assert!(!service.user_reset_password_exists(token).await.unwrap());
    }

    // ── reset_user_password ──

    #[tokio::test]
    async fn reset_user_password_success() {
        let mut auth_repo = MockAuthRepo::new();
        let user_repo = MockUserRepo::new();
        let email = MockEmailService::new();
        let token = Uuid::new_v4();

        auth_repo
            .expect_update_user_password()
            .returning(|_, _| Ok(()));

        let service = make_service(auth_repo, user_repo, email);
        assert!(
            service
                .reset_user_password(token, "newpass123".into())
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn reset_user_password_token_not_found() {
        let mut auth_repo = MockAuthRepo::new();
        let user_repo = MockUserRepo::new();
        let email = MockEmailService::new();
        let token = Uuid::new_v4();

        auth_repo
            .expect_update_user_password()
            .returning(|_, _| Err(sqlx::Error::RowNotFound));

        let service = make_service(auth_repo, user_repo, email);
        let result = service
            .reset_user_password(token, "newpass123".into())
            .await;

        assert_eq!(result.unwrap_err(), ErrorMessage::UserNoLongerExists);
    }

    // Helper: fake database error that reports a unique violation
    struct TestUniqueViolation;

    impl std::fmt::Display for TestUniqueViolation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "unique violation")
        }
    }

    impl std::fmt::Debug for TestUniqueViolation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestUniqueViolation")
        }
    }

    impl std::error::Error for TestUniqueViolation {}

    impl sqlx::error::DatabaseError for TestUniqueViolation {
        fn message(&self) -> &str {
            "unique violation"
        }

        fn kind(&self) -> sqlx::error::ErrorKind {
            sqlx::error::ErrorKind::UniqueViolation
        }

        fn as_error(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
            self
        }

        fn as_error_mut(&mut self) -> &mut (dyn std::error::Error + Send + Sync + 'static) {
            self
        }

        fn into_error(self: Box<Self>) -> Box<dyn std::error::Error + Send + Sync + 'static> {
            self
        }
    }
}
