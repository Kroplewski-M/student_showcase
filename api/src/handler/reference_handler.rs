use actix_web::{HttpResponse, dev::HttpServiceFactory, web};

use crate::{AppState, errors::HttpError};

pub fn reference_handler() -> impl HttpServiceFactory {
    web::scope("/ref").route("/link_types", web::get().to(get_link_types))
}

async fn get_link_types(app_state: web::Data<AppState>) -> Result<HttpResponse, HttpError> {
    const CACHE_KEY: &str = "link_types";

    //try get from cache
    if let Some(cached_value) = app_state.cache.get(CACHE_KEY).await {
        return Ok(HttpResponse::Ok().json(cached_value));
    }

    let res = app_state
        .reference_service
        .get_link_types()
        .await
        .map_err(HttpError::server_error)?;

    //set cache
    let value = serde_json::to_value(&res).map_err(|e| HttpError::server_error(e.to_string()))?;
    app_state
        .cache
        .insert(CACHE_KEY.to_string(), value.clone())
        .await;

    Ok(HttpResponse::Ok().json(value))
}
