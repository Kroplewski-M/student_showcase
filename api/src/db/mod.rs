use sqlx::{Pool, Postgres};
pub mod users_repo;

#[derive(Debug, Clone)]
pub struct DbClient {
    pub users: users_repo::UsersRepo,
}
impl DbClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            users: users_repo::UsersRepo::new(pool.clone()),
        }
    }
}
