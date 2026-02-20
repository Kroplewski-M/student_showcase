use sqlx::{Pool, Postgres};
pub mod auth_repo;

#[derive(Debug, Clone)]
pub struct DbClient {
    pub auth: auth_repo::AuthRepo,
}
impl DbClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            auth: auth_repo::AuthRepo::new(pool.clone()),
        }
    }
}
