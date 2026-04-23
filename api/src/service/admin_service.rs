use std::sync::Arc;

use crate::{db::admin_repo::AdminRepoTrait, dtos::admin::FindStudent, errors::ErrorMessage};

#[derive(Clone)]
pub struct AdminService {
    admin_repo: Arc<dyn AdminRepoTrait>,
}

impl AdminService {
    pub fn new(admin_repo: Arc<dyn AdminRepoTrait>) -> Self {
        Self { admin_repo }
    }
    pub async fn search_student(
        &self,
        student_id: String,
    ) -> Result<Option<FindStudent>, ErrorMessage> {
        self.admin_repo
            .search_student(&student_id)
            .await
            .map_err(|_| ErrorMessage::ServerError)
    }
    pub async fn suspend_student(&self, student_id: String) -> Result<(), ErrorMessage> {
        self.admin_repo
            .suspend_student(&student_id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ErrorMessage::UserNoLongerExists,
                _ => ErrorMessage::ServerError,
            })
    }
    pub async fn unsuspend_student(&self, student_id: String) -> Result<(), ErrorMessage> {
        self.admin_repo
            .unsuspend_student(&student_id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ErrorMessage::UserNoLongerExists,
                _ => ErrorMessage::ServerError,
            })
    }
}
