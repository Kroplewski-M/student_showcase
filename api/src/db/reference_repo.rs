use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::dtos::reference::LinkTypes;

#[derive(Debug, Clone)]
pub struct ReferenceRepo {
    pool: Pool<Postgres>,
}

impl ReferenceRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
pub trait ReferenceRepoTrait: Send + Sync {
    async fn get_link_types(&self) -> Result<Vec<LinkTypes>, sqlx::Error>;
}

#[async_trait]
impl ReferenceRepoTrait for ReferenceRepo {
    async fn get_link_types(&self) -> Result<Vec<LinkTypes>, sqlx::Error> {
        sqlx::query_as!(
            LinkTypes,
            r#"
            SELECT l.id, l.name FROM link_types l
        "#
        )
        .fetch_all(&self.pool)
        .await
    }
}
