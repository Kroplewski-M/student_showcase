use actix_web::{HttpResponse, dev::HttpServiceFactory, web};

use crate::{AppState, errors::HttpError, utils::generic::get_or_cache};

pub fn reference_handler() -> impl HttpServiceFactory {
    web::scope("/ref")
        .route("/link_types", web::get().to(get_link_types))
        .route("/courses", web::get().to(get_courses))
}

async fn get_link_types(app_state: web::Data<AppState>) -> Result<HttpResponse, HttpError> {
    const CACHE_KEY: &str = "link_types";

    get_or_cache(&app_state, CACHE_KEY, || {
        app_state.reference_service.get_link_types()
    })
    .await
}
async fn get_courses(app_state: web::Data<AppState>) -> Result<HttpResponse, HttpError> {
    const CACHE_KEY: &str = "all_courses";
    get_or_cache(&app_state, CACHE_KEY, || {
        app_state.reference_service.get_courses()
    })
    .await
}
