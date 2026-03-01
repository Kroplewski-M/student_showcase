use actix_web::{HttpResponse, dev::HttpServiceFactory, web};

use crate::{AppState, errors::HttpError};

pub fn reference_handler() -> impl HttpServiceFactory {
    web::scope("/ref")
        .route("/link_types", web::get().to(get_link_types))
        .route("/courses", web::get().to(get_courses))
        .route("/tools", web::get().to(get_tools))
}

async fn get_link_types(app_state: web::Data<AppState>) -> Result<HttpResponse, HttpError> {
    let res = app_state
        .reference_service
        .get_link_types()
        .await
        .map_err(HttpError::server_error)?;

    Ok(HttpResponse::Ok().json(res))
}
async fn get_courses(app_state: web::Data<AppState>) -> Result<HttpResponse, HttpError> {
    let res = app_state
        .reference_service
        .get_courses()
        .await
        .map_err(HttpError::server_error)?;

    Ok(HttpResponse::Ok().json(res))
}
async fn get_tools(app_state: web::Data<AppState>) -> Result<HttpResponse, HttpError> {
    let res = app_state
        .reference_service
        .get_tools()
        .await
        .map_err(HttpError::server_error)?;

    Ok(HttpResponse::Ok().json(res))
}
