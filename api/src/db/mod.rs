use sqlx::{Pool, Postgres};
pub mod users;

#[derive(Debug, Clone)]
pub struct DbClient {
    pub users: users::UsersRepo,
}
impl DbClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            users: users::UsersRepo::new(pool),
        }
    }
}
