use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct DbClient {
    pool: Pool<Postgres>,
}
impl DbClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}
