use moka::future::Cache;
use serde::de::DeserializeOwned;

use crate::errors::ErrorMessage;

pub fn get_email_for_student(student_id: &str) -> String {
    format!("U{student_id}@unimail.hud.ac.uk")
}

#[derive(Clone)]
pub struct MemoryCache {
    storage: Cache<String, serde_json::Value>,
}
impl MemoryCache {
    pub fn new(storage: Cache<String, serde_json::Value>) -> Self {
        Self { storage }
    }
    pub async fn get_or_cache<T, F, Fut>(
        &self,
        cache_key: &str,
        fetch: F,
    ) -> Result<T, ErrorMessage>
    where
        T: serde::Serialize + DeserializeOwned,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, ErrorMessage>>,
    {
        if let Some(cached_value) = self.storage.get(cache_key).await {
            return serde_json::from_value(cached_value).map_err(|_| ErrorMessage::ServerError);
        }

        let res = fetch().await.map_err(|_| ErrorMessage::ServerError)?;
        let value = serde_json::to_value(&res).map_err(|_| ErrorMessage::ServerError)?;

        self.storage
            .insert(cache_key.to_string(), value.clone())
            .await;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_email_correctly() {
        let email = get_email_for_student("1234567");
        assert_eq!(email, "U1234567@unimail.hud.ac.uk");
    }

    #[test]
    fn prepends_u_prefix() {
        let email = get_email_for_student("0000000");
        assert!(email.starts_with('U'));
    }

    #[test]
    fn uses_correct_domain() {
        let email = get_email_for_student("1234567");
        assert!(email.ends_with("@unimail.hud.ac.uk"));
    }

    #[test]
    fn empty_id_still_formats() {
        let email = get_email_for_student("");
        assert_eq!(email, "U@unimail.hud.ac.uk");
    }
}
