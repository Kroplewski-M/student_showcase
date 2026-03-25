use std::sync::Arc;

use actix_multipart::form::tempfile::TempFile;
use futures_util::future::try_join_all;
use tracing::error;
use uuid::Uuid;

use crate::{
    db::project_repo::ProjectRepoTrait,
    dtos::{
        reference::FileInfo,
        user::{ProjectForm, ProjectFormData, ProjectUpsertData, UpsertProjectParams},
    },
    errors::ErrorMessage,
    models::file::File,
    service::reference_service::ReferenceService,
    utils::{
        embedding::Embedding,
        file_storage::FileStorageTrait,
        images::{DEFAULT_MAX_IMAGE_SIZE, ValidatedImage},
    },
};

#[derive(Clone)]
pub struct ProjectService {
    project_repo: Arc<dyn ProjectRepoTrait>,
    project_file_storage: Arc<dyn FileStorageTrait>,
    embedding: Arc<Embedding>,
    reference_service: ReferenceService,
}

pub static MAX_IMAGES: usize = 5;
impl ProjectService {
    pub fn new(
        project_repo: Arc<dyn ProjectRepoTrait>,
        project_file_storage: Arc<dyn FileStorageTrait>,
        embedding: Arc<Embedding>,
        reference_service: ReferenceService,
    ) -> Self {
        Self {
            project_repo,
            project_file_storage,
            embedding,
            reference_service,
        }
    }
    pub async fn get_user_project_form_data(
        &self,
        user_id: String,
        project_id: Option<Uuid>,
    ) -> Result<ProjectForm, ErrorMessage> {
        let tools = self.reference_service.get_tools().await?;
        let link_types = self.reference_service.get_link_types().await?;
        let mut data: ProjectFormData = ProjectFormData::default();
        if let Some(proj_id) = project_id {
            data = self
                .project_repo
                .get_user_project_form_data(&user_id, proj_id)
                .await
                .map_err(|e| match e {
                    sqlx::Error::RowNotFound => ErrorMessage::ProjectNotFound,
                    _ => ErrorMessage::ServerError,
                })?;
        }
        Ok(ProjectForm {
            project: data,
            tools_list: tools,
            link_types,
        })
    }
    pub async fn upsert_user_project(
        &self,
        user_id: String,
        data: ProjectUpsertData,
        new_images: Vec<TempFile>,
    ) -> Result<(), ErrorMessage> {
        //max images
        if data.existing_images.len() + new_images.len() > MAX_IMAGES {
            return Err(ErrorMessage::TooManyFiles(MAX_IMAGES));
        }
        //get tools
        let tools = self.reference_service.get_tools().await?;
        let tool_names: Vec<String> = data
            .selected_tools
            .iter()
            .filter_map(|id| tools.iter().find(|t| t.id == *id))
            .map(|t| t.name.clone())
            .collect();
        let project_embedding_document = data.to_embedding_document(&tool_names);
        let embedding = self
            .embedding
            .embed_document(project_embedding_document)
            .await?;

        let vector = pgvector::Vector::from(embedding);
        //get all current project images
        let current_files = if let Some(id) = &data.id {
            self.project_repo
                .get_project_files(id)
                .await
                .map_err(|_| ErrorMessage::ServerError)?
        } else {
            Vec::<File>::new()
        };
        //upload new images to storage
        let validated_images: Vec<ValidatedImage> =
            try_join_all(new_images.into_iter().map(|f| async move {
                let file_name = f.file_name.unwrap_or_default();
                let bytes = tokio::fs::read(f.file.path())
                    .await
                    .map_err(|_| ErrorMessage::ServerError)?;
                ValidatedImage::from_bytes(file_name, bytes, DEFAULT_MAX_IMAGE_SIZE)
            }))
            .await?;

        let mut uploaded_images = Vec::<FileInfo>::with_capacity(validated_images.len());
        for file in validated_images {
            let new_name = file.generate_new_filename();
            let disk_filename = file.full_name(&new_name);
            if self
                .project_file_storage
                .write(disk_filename.as_str(), file.bytes())
                .await
                .is_err()
            {
                for f in &uploaded_images {
                    let name = format!("{}.{}", f.new_name, f.extension);
                    if let Err(e) = self.project_file_storage.delete(&name).await {
                        error!(
                            "Failed to delete uploaded project image during rollback {}: {}",
                            name, e
                        );
                    }
                }
                return Err(ErrorMessage::ServerError);
            }
            uploaded_images.push(FileInfo {
                new_name,
                old_name: file.old_name(),
                length: file.len(),
                file_type: file.format().mime_type().to_string(),
                extension: file.format().extension().to_string(),
            });
        }

        // Disk names of newly uploaded files — needed for rollback if DB fails
        let uploaded_disk_names: Vec<String> = uploaded_images
            .iter()
            .map(|f| format!("{}.{}", f.new_name, f.extension))
            .collect();

        // Current files no longer in existing_images — delete from storage on success
        let stale_files: Vec<String> = current_files
            .iter()
            .filter(|f| !data.existing_images.contains(&f.get_full_name()))
            .map(|f| f.get_full_name())
            .collect();
        let params = UpsertProjectParams {
            user_id,
            project_id: data.id,
            name: data.name,
            description: data.description,
            live_link: data.live_link,
            selected_tools: data.selected_tools,
            links: data.links,
            new_images: uploaded_images,
            existing_images: data.existing_images,
            embedding: vector,
        };
        let res = self.project_repo.upsert_project(params).await;

        match res {
            Ok(_) => {
                for name in stale_files {
                    if let Err(e) = self.project_file_storage.delete(&name).await {
                        error!("Failed to delete stale project image {}: {}", name, e);
                    }
                }
            }
            Err(_) => {
                for name in uploaded_disk_names {
                    if let Err(e) = self.project_file_storage.delete(&name).await {
                        error!("Failed to delete uploaded project image {}: {}", name, e);
                    }
                }
                return Err(ErrorMessage::ServerError);
            }
        }

        Ok(())
    }
    pub async fn delete_project(
        &self,
        user_id: String,
        project_id: Uuid,
    ) -> Result<(), ErrorMessage> {
        //get current project images
        let current_images = self
            .project_repo
            .get_project_files(&project_id)
            .await
            .map_err(|_| ErrorMessage::ServerError)?;
        //delte project
        self.project_repo
            .delete_project(&user_id, project_id)
            .await
            .map_err(|_| ErrorMessage::ServerError)?;
        //remove files from storage
        for file in current_images {
            if let Err(e) = self
                .project_file_storage
                .delete(&file.get_full_name())
                .await
            {
                error!(
                    "Failed to delete project image {}: {}",
                    file.get_full_name(),
                    e
                );
            }
        }
        Ok(())
    }
    pub async fn feature_project(
        &self,
        user_id: String,
        project_id: Uuid,
    ) -> Result<(), ErrorMessage> {
        self.project_repo
            .feature_project(&user_id, project_id)
            .await
            .map_err(|_| ErrorMessage::ServerError)
    }
}
