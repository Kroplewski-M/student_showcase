use sqlx::{Pool, Postgres};
pub mod auth_repo;
pub mod user_repo;

#[derive(Debug, Clone)]
pub struct DbClient {
    pub auth: auth_repo::AuthRepo,
    pub user: user_repo::UserRepo,
}
impl DbClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            auth: auth_repo::AuthRepo::new(pool.clone()),
            user: user_repo::UserRepo::new(pool.clone()),
        }
    }
}
