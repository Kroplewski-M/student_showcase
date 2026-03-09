use actix_multipart::Multipart;
use actix_web::{HttpResponse, dev::HttpServiceFactory, web};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    dtos::{
        Response,
        user::{UpdateUserInfo, UpsertProjectQuery, UserProfileForm},
    },
    errors::{ErrorMessage, HttpError},
    middleware::auth::{AuthenticatedUserId, RequireAuth},
    models::file::FormFile,
};

pub fn user_handler() -> impl HttpServiceFactory {
    web::scope("/user")
        // Public routes (no auth)
        .route("/info/{id}", web::get().to(get_user_profile))
        // Protected routes wrapped in their own scope
        .service(
            web::scope("")
                .wrap(RequireAuth)
                .route("/update_image", web::post().to(update_user_image))
                .route("/update_profile", web::get().to(get_user_profile_form))
                .route("/update_profile", web::patch().to(patch_user_profile))
                .route("/upsert_project", web::get().to(upsert_user_project)),
        )
}

pub async fn get_user_profile(
    app_state: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, HttpError> {
    let user = app_state
        .user_service
        .get_user_profile(id.to_string())
        .await
        .map_err(|e| match e {
            ErrorMessage::UserNoLongerExists => HttpError::not_found(e),
            _ => HttpError::server_error(e),
        })?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn update_user_image(
    app_state: web::Data<AppState>,
    user_id: AuthenticatedUserId,
    payload: Multipart,
) -> Result<HttpResponse, HttpError> {
    let file_data = FormFile::new_from_form_multi_part(payload)
        .await
        .map_err(HttpError::bad_request)?;

    app_state
        .user_service
        .update_user_image(user_id.to_string(), file_data.bytes, file_data.name)
        .await
        .map_err(|e| match e {
            ErrorMessage::ServerError => HttpError::server_error(e),
            _ => HttpError::bad_request(e),
        })?;

    Ok(HttpResponse::Ok().json(Response {
        status: "success",
        message: "user updated profile image".to_string(),
    }))
}

pub async fn get_user_profile_form(
    app_state: web::Data<AppState>,
    user_id: AuthenticatedUserId,
) -> Result<HttpResponse, HttpError> {
    let (form_data, courses, link_types, tools) = tokio::try_join!(
        app_state
            .user_service
            .get_user_form_data(user_id.to_string()),
        app_state.reference_service.get_courses(),
        app_state.reference_service.get_link_types(),
        app_state.reference_service.get_tools(),
    )
    .map_err(|e| match e {
        ErrorMessage::UserNoLongerExists => HttpError::not_found(e),
        _ => HttpError::server_error(e),
    })?;

    Ok(HttpResponse::Ok().json(UserProfileForm {
        user_data: form_data,
        courses_list: courses,
        link_types,
        tools_list: tools,
    }))
}
pub async fn patch_user_profile(
    app_state: web::Data<AppState>,
    user_id: AuthenticatedUserId,
    data: web::Json<UpdateUserInfo>,
) -> Result<HttpResponse, HttpError> {
    data.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;
    app_state
        .user_service
        .update_user(user_id.to_string(), data.0)
        .await
        .map_err(|e| match e {
            ErrorMessage::UserNoLongerExists => HttpError::not_found("user not found"),
            _ => HttpError::server_error(e),
        })?;
    Ok(HttpResponse::Ok().json(Response {
        status: "success",
        message: "User updated successfully".to_string(),
    }))
}
pub async fn upsert_user_project(
    app_state: web::Data<AppState>,
    user_id: AuthenticatedUserId,
    query: web::Query<UpsertProjectQuery>,
) -> Result<HttpResponse, HttpError> {
    let proj_id = query
        .project_id
        .as_deref()
        .map(Uuid::try_parse)
        .transpose()
        .map_err(|_| HttpError::bad_request("invalid project id"))?;

    let data = app_state
        .user_service
        .get_user_project_form_data(user_id.to_string(), proj_id)
        .await
        .map_err(HttpError::server_error)?;
    Ok(HttpResponse::Ok().json(data))
}
