use std::sync::Arc;

use crate::{
    db::reference_repo::ReferenceRepoTrait,
    dtos::reference::{Course, LinkType, SoftwareTool},
    errors::ErrorMessage,
    utils::generic::MemoryCache,
};

#[derive(Clone)]
pub struct ReferenceService {
    reference_repo: Arc<dyn ReferenceRepoTrait>,
    cache: MemoryCache,
}

impl ReferenceService {
    pub fn new(reference_repo: Arc<dyn ReferenceRepoTrait>, cache: MemoryCache) -> Self {
        Self {
            reference_repo,
            cache,
        }
    }
}
impl ReferenceService {
    pub async fn get_link_types(&self) -> Result<Vec<LinkType>, ErrorMessage> {
        const CACHE_KEY: &str = "link_types";
        self.cache
            .get_or_cache(CACHE_KEY, || async {
                self.reference_repo
                    .get_link_types()
                    .await
                    .map_err(|_| ErrorMessage::ServerError)
            })
            .await
    }
    pub async fn get_courses(&self) -> Result<Vec<Course>, ErrorMessage> {
        const CACHE_KEY: &str = "courses_list";
        self.cache
            .get_or_cache(CACHE_KEY, || async {
                self.reference_repo
                    .get_courses()
                    .await
                    .map_err(|_| ErrorMessage::ServerError)
            })
            .await
    }
    pub async fn get_tools(&self) -> Result<Vec<SoftwareTool>, ErrorMessage> {
        const CACHE_KEY: &str = "software_tool_list";
        self.cache
            .get_or_cache(CACHE_KEY, || async {
                self.reference_repo
                    .get_tools()
                    .await
                    .map_err(|_| ErrorMessage::ServerError)
            })
            .await
    }
}
