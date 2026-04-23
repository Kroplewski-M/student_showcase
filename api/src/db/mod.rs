use std::sync::Arc;

use sqlx::{Pool, Postgres};
pub mod admin_repo;
pub mod auth_repo;
pub mod project_repo;
pub mod reference_repo;
pub mod user_repo;

#[derive(Clone)]
pub struct DbClient {
    pub auth: auth_repo::AuthRepo,
    pub user: user_repo::UserRepo,
    pub reference: reference_repo::ReferenceRepo,
    pub project: project_repo::ProjectRepo,
    pub admin: admin_repo::AdminRepo,
}
impl DbClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        let user_repo = user_repo::UserRepo::new(pool.clone());
        Self {
            auth: auth_repo::AuthRepo::new(pool.clone(), Arc::new(user_repo.clone())),
            user: user_repo,
            reference: reference_repo::ReferenceRepo::new(pool.clone()),
            project: project_repo::ProjectRepo::new(pool.clone()),
            admin: admin_repo::AdminRepo::new(pool.clone()),
        }
    }
}
