use std::sync::Arc;

use sqlx::{Pool, Postgres};
pub mod auth_repo;
pub mod user_repo;

#[derive(Clone)]
pub struct DbClient {
    pub auth: auth_repo::AuthRepo,
    pub user: user_repo::UserRepo,
}
impl DbClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        let user_repo = user_repo::UserRepo::new(pool.clone());
        Self {
            auth: auth_repo::AuthRepo::new(pool.clone(), Arc::new(user_repo.clone())),
            user: user_repo,
        }
    }
}
