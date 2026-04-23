use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AdminRepo {
    pool: Pool<Postgres>,
}

impl AdminRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
pub trait AdminRepoTrait: Send + Sync {}

#[async_trait]
impl AdminRepoTrait for AdminRepo {}
