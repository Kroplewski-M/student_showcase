use actix_web::{HttpResponse, web};

use crate::{AppState, errors::HttpError};

pub fn get_email_for_student(student_id: &str) -> String {
    format!("U{student_id}@unimail.hud.ac.uk")
}

pub async fn get_or_cache<T, F, Fut, E>(
    app_state: &web::Data<AppState>,
    cache_key: &str,
    fetch: F,
) -> Result<HttpResponse, HttpError>
where
    T: serde::Serialize,
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: ToString,
{
    if let Some(cached_value) = app_state.cache.get(cache_key).await {
        return Ok(HttpResponse::Ok().json(cached_value));
    }
    let res = fetch()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let value = serde_json::to_value(&res).map_err(|e| HttpError::server_error(e.to_string()))?;

    app_state
        .cache
        .insert(cache_key.to_string(), value.clone())
        .await;
    Ok(HttpResponse::Ok().json(value))
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
