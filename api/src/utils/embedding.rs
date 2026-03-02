use std::sync::{Arc, Mutex};

use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use tracing::info;

use crate::errors::ErrorMessage;

pub struct Embedding {
    pool: Vec<Arc<Mutex<TextEmbedding>>>,
}

impl Embedding {
    pub fn new(cpu_count: usize) -> Result<Self, ErrorMessage> {
        // Use half the cores as pool size — ONNX Runtime uses multiple threads
        // per inference internally, so fewer instances avoids over-subscribing the CPU
        let pool_size = (cpu_count / 2).max(1);

        let pool = (0..pool_size)
            .map(|_| {
                TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2))
                    .map(|m| Arc::new(Mutex::new(m)))
                    .map_err(|_| ErrorMessage::EmbeddingFailed)
            })
            .collect::<Result<Vec<_>, ErrorMessage>>()?;

        info!(
            "embedding pool: {} instances ({} total cores)",
            pool_size, cpu_count
        );
        Ok(Self { pool })
    }

    pub async fn embed_document(&self, document: String) -> Result<Vec<f32>, ErrorMessage> {
        let model = self
            .pool
            .iter()
            .find(|m| m.try_lock().is_ok())
            .unwrap_or(&self.pool[0]);

        let model = Arc::clone(model);
        tokio::task::spawn_blocking(move || {
            model
                .lock()
                .map_err(|_| ErrorMessage::EmbeddingFailed)?
                .embed(vec![document], None)
                .map_err(|_| ErrorMessage::EmbeddingFailed)?
                .into_iter()
                .next()
                .ok_or(ErrorMessage::EmbeddingFailed)
        })
        .await
        .map_err(|_| ErrorMessage::EmbeddingFailed)?
    }
}
