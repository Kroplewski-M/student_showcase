use std::sync::{Arc, Mutex};

use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use tracing::info;

use crate::errors::ErrorMessage;

pub struct Embedding {
    pool: Vec<Arc<Mutex<TextEmbedding>>>,
}

#[cfg(test)]
impl Embedding {
    fn pool_len(&self) -> usize {
        self.pool.len()
    }
}

impl Embedding {
    pub fn new(cpu_count: usize) -> Result<Self, ErrorMessage> {
        // Use half the cores as pool size — ONNX Runtime uses multiple threads
        // per inference internally, so fewer instances avoids over-subscribing the CPU
        // added max of 4 to not have too many
        let mut pool_size = (cpu_count / 2).max(1);
        if pool_size > 4 {
            pool_size = 4;
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_size_zero_cores_clamps_to_one() {
        let e = Embedding::new(0).unwrap();
        assert_eq!(e.pool_len(), 1);
    }
    #[test]
    fn pool_size_four_cores_is_two() {
        let e = Embedding::new(4).unwrap();
        assert_eq!(e.pool_len(), 2);
    }

    // ─── embed_document ──────────────────────────────────────────────

    #[tokio::test]
    async fn embed_document_returns_ok() {
        let e = Embedding::new(2).unwrap();
        let result = e.embed_document("hello world".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn embed_document_produces_384_dimensions() {
        // AllMiniLML6V2 output dimension is 384
        let e = Embedding::new(2).unwrap();
        let vec = e.embed_document("test".to_string()).await.unwrap();
        assert_eq!(vec.len(), 384);
    }

    #[tokio::test]
    async fn embed_document_is_deterministic() {
        let e = Embedding::new(2).unwrap();
        let doc = "deterministic input".to_string();
        let v1 = e.embed_document(doc.clone()).await.unwrap();
        let v2 = e.embed_document(doc).await.unwrap();
        assert_eq!(v1, v2);
    }

    #[tokio::test]
    async fn embed_document_different_inputs_differ() {
        let e = Embedding::new(2).unwrap();
        let v1 = e.embed_document("cats".to_string()).await.unwrap();
        let v2 = e.embed_document("dogs".to_string()).await.unwrap();
        assert_ne!(v1, v2);
    }

    #[tokio::test]
    async fn embed_document_empty_string_returns_vector() {
        let e = Embedding::new(2).unwrap();
        let result = e.embed_document(String::new()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 384);
    }

    #[tokio::test]
    async fn embed_document_similar_sentences_have_high_cosine_similarity() {
        let e = Embedding::new(2).unwrap();
        let v1 = e
            .embed_document("The dog ran across the field".to_string())
            .await
            .unwrap();
        let v2 = e
            .embed_document("A dog was running through a meadow".to_string())
            .await
            .unwrap();

        let dot: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
        let norm1: f32 = v1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = v2.iter().map(|x| x * x).sum::<f32>().sqrt();
        let cosine = dot / (norm1 * norm2);

        // Semantically similar sentences should score above 0.65
        assert!(cosine > 0.65, "expected high similarity, got {cosine}");
    }

    #[tokio::test]
    async fn embed_document_dissimilar_sentences_have_lower_cosine_similarity() {
        let e = Embedding::new(2).unwrap();
        let v1 = e
            .embed_document("The stock market crashed today".to_string())
            .await
            .unwrap();
        let v2 = e
            .embed_document("Penguins live in Antarctica".to_string())
            .await
            .unwrap();

        let dot: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
        let norm1: f32 = v1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = v2.iter().map(|x| x * x).sum::<f32>().sqrt();
        let cosine = dot / (norm1 * norm2);

        // Unrelated sentences should score below the similar pair
        assert!(cosine < 0.9, "expected lower similarity, got {cosine}");
    }
}
