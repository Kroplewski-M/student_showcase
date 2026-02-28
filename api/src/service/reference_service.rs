use std::sync::Arc;

use crate::{
    db::reference_repo::ReferenceRepoTrait, dtos::reference::LinkType, errors::ErrorMessage,
};

#[derive(Clone)]
pub struct ReferenceService {
    reference_repo: Arc<dyn ReferenceRepoTrait>,
}

impl ReferenceService {
    pub fn new(reference_repo: Arc<dyn ReferenceRepoTrait>) -> Self {
        Self { reference_repo }
    }
}
impl ReferenceService {
    pub async fn get_link_types(&self) -> Result<Vec<LinkType>, ErrorMessage> {
        self.reference_repo
            .get_link_types()
            .await
            .map_err(|_| ErrorMessage::ServerError)
    }
}
