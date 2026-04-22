use crate::{
    AppState,
    dtos::{
        Response,
        user::{ProjectFormUpsert, UpsertProjectQuery},
    },
    errors::{ErrorMessage, HttpError},
    middleware::auth::{AuthenticatedUserId, RequireAuth},
};
use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, dev::HttpServiceFactory, web};
use uuid::Uuid;
use validator::Validate;

pub fn project_handler() -> impl HttpServiceFactory {
    web::scope("/project").service(
        web::scope("")
            .wrap(RequireAuth::default())
            .route("/upsert_project", web::get().to(get_user_project_form))
            .route("/upsert_project", web::post().to(post_user_project_form))
            .route(
                "/delete_project/{project_id}",
                web::delete().to(delete_user_project),
            )
            .route(
                "/feature_project/{project_id}",
                web::post().to(feature_user_project),
            ),
    )
}
pub async fn get_user_project_form(
    app_state: web::Data<AppState>,
    user_id: AuthenticatedUserId,
    query: web::Query<UpsertProjectQuery>,
) -> Result<HttpResponse, HttpError> {
    let data = app_state
        .project_service
        .get_user_project_form_data(user_id.to_string(), query.project_id)
        .await
        .map_err(|e| match e {
            ErrorMessage::ProjectNotFound => HttpError::not_found(e),
            _ => HttpError::server_error(e),
        })?;
    Ok(HttpResponse::Ok().json(data))
}
pub async fn post_user_project_form(
    app_state: web::Data<AppState>,
    user_id: AuthenticatedUserId,
    MultipartForm(form): MultipartForm<ProjectFormUpsert>,
) -> Result<HttpResponse, HttpError> {
    let data = form.data.into_inner();
    data.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let res = app_state
        .project_service
        .upsert_user_project(user_id.to_string(), data, form.new_files)
        .await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(Response {
            status: "success",
            message: "project updated successfully".to_string(),
        })),
        Err(e) => match e {
            ErrorMessage::FileSizeTooBig(_)
            | ErrorMessage::TooManyFiles(_)
            | ErrorMessage::FileInvalidFormat(_)
            | ErrorMessage::FileInvalidName => Err(HttpError::bad_request(e.to_string())),
            _ => Err(HttpError::server_error(e.to_string())),
        },
    }
}
pub async fn delete_user_project(
    app_state: web::Data<AppState>,
    user_id: AuthenticatedUserId,
    project_id: web::Path<Uuid>,
) -> Result<HttpResponse, HttpError> {
    app_state
        .project_service
        .delete_project(user_id.to_string(), project_id.to_owned())
        .await
        .map_err(HttpError::server_error)?;
    Ok(HttpResponse::Ok().json(Response {
        status: "success",
        message: "project deleted successfully".to_string(),
    }))
}
pub async fn feature_user_project(
    app_state: web::Data<AppState>,
    user_id: AuthenticatedUserId,
    project_id: web::Path<Uuid>,
) -> Result<HttpResponse, HttpError> {
    app_state
        .project_service
        .feature_project(user_id.to_string(), project_id.to_owned())
        .await
        .map_err(HttpError::server_error)?;
    Ok(HttpResponse::Ok().json(Response {
        status: "success",
        message: "project updated successfully".to_string(),
    }))
}
