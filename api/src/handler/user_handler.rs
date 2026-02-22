use actix_multipart::Multipart;
use actix_web::{HttpResponse, dev::HttpServiceFactory, web};

use crate::{
    AppState,
    dtos::Response,
    errors::HttpError,
    middleware::auth::{AuthenticatedUserId, RequireAuth},
    models::FormFile,
};

pub fn user_handler() -> impl HttpServiceFactory {
    web::scope("/user")
        .wrap(RequireAuth)
        .route("/update_image", web::post().to(update_user_image))
}

pub async fn update_user_image(
    app_state: web::Data<AppState>,
    user_id: AuthenticatedUserId,
    payload: Multipart,
) -> Result<HttpResponse, HttpError> {
    let file_data = FormFile::new_from_form_muli_part(payload)
        .await
        .map_err(HttpError::server_error)?;

    app_state
        .user_service
        .update_user_image(user_id.to_string(), file_data.bytes, file_data.name)
        .await
        .map_err(HttpError::server_error)?;

    Ok(HttpResponse::Ok().json(Response {
        status: "success",
        message: "user updated profile image".to_string(),
    }))
}
