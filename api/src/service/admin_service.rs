use std::sync::Arc;

use crate::db::admin_repo::AdminRepoTrait;

#[derive(Clone)]
pub struct AdminService {
    admin_repo: Arc<dyn AdminRepoTrait>,
}

impl AdminService {
    pub fn new(admin_repo: Arc<dyn AdminRepoTrait>) -> Self {
        Self { admin_repo }
    }
}
