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
        let value = self
            .storage
            .try_get_with(cache_key.to_string(), async {
                let res = fetch().await?;
                serde_json::to_value(&res).map_err(|_| ErrorMessage::ServerError)
            })
            .await
            .map_err(|e| (*e).clone())?;

        serde_json::from_value(value).map_err(|_| ErrorMessage::ServerError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cache() -> MemoryCache {
        MemoryCache::new(Cache::builder().max_capacity(100).build())
    }

    #[tokio::test]
    async fn get_or_cache_miss_fetches_and_returns_value() {
        let cache = make_cache();
        let result = cache
            .get_or_cache("key", || async { Ok::<String, ErrorMessage>("hello".to_string()) })
            .await;
        assert_eq!(result.unwrap(), "hello");
    }

    #[tokio::test]
    async fn get_or_cache_hit_returns_cached_value_without_calling_fetch() {
        let cache = make_cache();
        cache
            .get_or_cache("key", || async { Ok::<String, ErrorMessage>("cached".to_string()) })
            .await
            .unwrap();
        let result = cache
            .get_or_cache("key", || async { Ok::<String, ErrorMessage>("new".to_string()) })
            .await;
        assert_eq!(result.unwrap(), "cached");
    }

    #[tokio::test]
    async fn get_or_cache_propagates_fetch_error() {
        let cache = make_cache();
        let result = cache
            .get_or_cache::<String, _, _>("key", || async { Err(ErrorMessage::ServerError) })
            .await;
        assert_eq!(result.unwrap_err(), ErrorMessage::ServerError);
    }

    #[tokio::test]
    async fn get_or_cache_different_keys_do_not_interfere() {
        let cache = make_cache();
        cache
            .get_or_cache("key1", || async { Ok::<String, ErrorMessage>("value1".to_string()) })
            .await
            .unwrap();
        cache
            .get_or_cache("key2", || async { Ok::<String, ErrorMessage>("value2".to_string()) })
            .await
            .unwrap();

        let r1 = cache
            .get_or_cache("key1", || async { Ok::<String, ErrorMessage>("x".to_string()) })
            .await;
        let r2 = cache
            .get_or_cache("key2", || async { Ok::<String, ErrorMessage>("x".to_string()) })
            .await;

        assert_eq!(r1.unwrap(), "value1");
        assert_eq!(r2.unwrap(), "value2");
    }

    #[tokio::test]
    async fn get_or_cache_works_with_complex_types() {
        let cache = make_cache();
        let result = cache
            .get_or_cache("key", || async { Ok::<Vec<u32>, ErrorMessage>(vec![1, 2, 3]) })
            .await;
        assert_eq!(result.unwrap(), vec![1u32, 2, 3]);
    }

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
