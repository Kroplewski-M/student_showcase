use actix_web::{HttpResponse, Responder, Scope, cookie::Cookie, web};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    dtos::{Response, auth::RegisterUserDto},
    errors::{ErrorMessage, HttpError},
    middleware::auth::RequireAuth,
};

pub fn auth_handler() -> Scope {
    web::scope("/auth")
        .route("/login", web::post().to(login))
        .route("/register", web::post().to(register))
        .route("/validate-user/{token}", web::post().to(validate_user))
        .route("/reset-password", web::post().to(reset_password))
        .route("/logout", web::post().to(logout).wrap(RequireAuth))
}

pub async fn login() -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok().body("ok"))
}

pub async fn register(
    app_state: web::Data<AppState>,
    body: web::Json<RegisterUserDto>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    match app_state
        .auth_service
        .register(body.id.to_string(), body.password.to_string())
        .await
    {
        Ok(_) => Ok(HttpResponse::Created().json(Response {
            status: "success",
            message: "user registered successfully".to_string(),
        })),
        Err(ErrorMessage::UserAlreadyExists) => Err(HttpError::unique_constraint_voilation(
            ErrorMessage::UserAlreadyExists,
        )),
        Err(e) => Err(HttpError::server_error(e)),
    }
}
pub async fn validate_user(
    app_state: web::Data<AppState>,
    token: web::Path<Uuid>,
) -> Result<HttpResponse, HttpError> {
    match app_state
        .auth_service
        .validate_user(token.into_inner())
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().json(Response {
            status: "success",
            message: "user verified successfully".to_string(),
        })),
        Err(e) => match &e {
            ErrorMessage::VerifyTokenDoesNotExist => Err(HttpError::bad_request(e)),
            _ => Err(HttpError::server_error(e)),
        },
    }
}
pub async fn reset_password() -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok().body("ok"))
}
pub async fn logout(app_state: web::Data<AppState>) -> impl Responder {
    let is_prod = std::env::var("RUST_ENV").unwrap_or_default() == "production";

    let cookie = Cookie::build(&app_state.config.auth_cookie_name, "")
        .path("/")
        .max_age(actix_web::cookie::time::Duration::new(-1, 0))
        .http_only(is_prod)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}
