use actix_web::{HttpResponse, dev::HttpServiceFactory, web};

use crate::{
    AppState,
    dtos::{Response, auth::validate_student_id},
    errors::{ErrorMessage, HttpError},
    middleware::auth::RequireAuth,
};

pub fn admin_handler() -> impl HttpServiceFactory {
    web::scope("/admin").service(
        web::scope("")
            .wrap(RequireAuth::admin())
            .route(
                "/search_student/{student_id}",
                web::get().to(search_student),
            )
            .route(
                "/suspend_student/{student_id}",
                web::post().to(suspend_student),
            )
            .route(
                "/unsuspend_student/{student_id}",
                web::post().to(unsuspend_student),
            ),
    )
}

pub async fn search_student(
    app_state: web::Data<AppState>,
    student_id: web::Path<String>,
) -> Result<HttpResponse, HttpError> {
    validate_student_id(&student_id).map_err(|e| HttpError::bad_request(e.to_string()))?;
    let res = app_state
        .admin_service
        .search_student(student_id.into_inner())
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(HttpResponse::Ok().json(res))
}
pub async fn suspend_student(
    app_state: web::Data<AppState>,
    student_id: web::Path<String>,
) -> Result<HttpResponse, HttpError> {
    validate_student_id(&student_id).map_err(|e| HttpError::bad_request(e.to_string()))?;
    app_state
        .admin_service
        .suspend_student(student_id.into_inner())
        .await
        .map_err(|e| match e {
            ErrorMessage::UserNoLongerExists => HttpError::bad_request(e.to_string()),
            _ => HttpError::server_error(e.to_string()),
        })?;

    Ok(HttpResponse::Ok().json(Response {
        status: "success",
        message: "user suspended".to_string(),
    }))
}
pub async fn unsuspend_student(
    app_state: web::Data<AppState>,
    student_id: web::Path<String>,
) -> Result<HttpResponse, HttpError> {
    validate_student_id(&student_id).map_err(|e| HttpError::bad_request(e.to_string()))?;
    app_state
        .admin_service
        .unsuspend_student(student_id.into_inner())
        .await
        .map_err(|e| match e {
            ErrorMessage::UserNoLongerExists => HttpError::bad_request(e.to_string()),
            _ => HttpError::server_error(e.to_string()),
        })?;

    Ok(HttpResponse::Ok().json(Response {
        status: "success",
        message: "user unsuspended".to_string(),
    }))
}
