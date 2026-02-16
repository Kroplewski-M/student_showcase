use crate::errors::{ErrorMessage, ErrorResponse, HttpError};
use crate::{AppState, utils};
use actix_web::cookie::Cookie;
use actix_web::error::{ErrorForbidden, ErrorInternalServerError};
use actix_web::web;
use actix_web::{
    FromRequest, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    http,
};
use futures_util::FutureExt;
use futures_util::future::{LocalBoxFuture, Ready, ready};
use serde::Serialize;
use std::ops::Deref;
use std::rc::Rc;

/// Newtype wrapper around a user ID that has already been authenticated.
/// This is what handlers will extract once authentication succeeds.
#[derive(Clone, Serialize)]
pub struct AuthenticatedUserId(pub String);

impl Deref for AuthenticatedUserId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Allows `AuthenticatedUserId` to be extracted in handlers like:
/// `fn handler(user_id: AuthenticatedUserId) -> impl Responder`
impl FromRequest for AuthenticatedUserId {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        ready(
            req.extensions()
                .get::<AuthenticatedUserId>()
                .cloned()
                .ok_or_else(|| ErrorUnauthorized(HttpError::unauthorized("Authentication Error"))),
        )
    }
}

/// Middleware struct.
/// Wraps the inner service (next handler/middleware in the chain).
pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

/// Implementation of the actual middleware logic.
impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse<actix_web::body::BoxBody>,
            Error = actix_web::Error,
        > + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, actix_web::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let app_state = req.app_data::<web::Data<AppState>>().unwrap().clone();
        let token = req
            .cookie(app_state.config.auth_cookie_name.as_str())
            .map(|c| c.value().to_owned())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .and_then(|h| h.to_str().ok())
                    .and_then(|h| h.strip_prefix("Bearer "))
                    .map(str::to_owned)
            });
        if token.is_none() {
            let json_error = ErrorResponse {
                status: "fail".to_string(),
                message: ErrorMessage::TokenNotProvided.to_string(),
            };
            return Box::pin(ready(Err(ErrorUnauthorized(json_error))));
        }

        let token_info = match utils::token::decode_token(
            token.unwrap(),
            app_state.config.jwt_secret.as_bytes(),
        ) {
            Ok(id) => id,
            Err(e) => {
                return Box::pin(ready(Err(ErrorUnauthorized(ErrorResponse {
                    status: "fail".to_string(),
                    message: e.message,
                }))));
            }
        };
        let cloned_app_state = app_state.clone();
        let srv = Rc::clone(&self.service);

        async move {
            let user_id = token_info.sub.to_string();
            let exists = cloned_app_state
                .db_client
                .users
                .exists_verified(&user_id)
                .await
                .map_err(|e| ErrorInternalServerError(HttpError::server_error(e.to_string())))?;

            if !exists {
                return Err(ErrorUnauthorized(ErrorResponse {
                    status: "fail".into(),
                    message: ErrorMessage::PermissionDenied.to_string(),
                }));
            }

            req.extensions_mut()
                .insert(AuthenticatedUserId(user_id.clone()));
            let mut response = srv.call(req).await?;

            //Refresh the token
            let already_set_cookie = response
                .response()
                .cookies()
                .any(|c| c.name() == app_state.config.auth_cookie_name);
            let should_renew = !already_set_cookie && {
                let age = chrono::Utc::now().timestamp() - token_info.iat;
                age > 60
            };
            if should_renew {
                let new_token = utils::token::create_token(
                    &user_id,
                    cloned_app_state.config.jwt_secret.as_bytes(),
                    cloned_app_state.config.jwt_max_age_mins,
                )
                .map_err(|e| ErrorInternalServerError(HttpError::server_error(e.to_string())))?;

                let cookie = Cookie::build(&app_state.config.auth_cookie_name, new_token)
                    .path("/")
                    .http_only(true)
                    .secure(app_state.config.is_prod)
                    .same_site(actix_web::cookie::SameSite::Lax)
                    .max_age(actix_web::cookie::time::Duration::minutes(
                        app_state.config.jwt_max_age_mins,
                    ))
                    .finish();
                response.response_mut().add_cookie(&cookie).map_err(|e| {
                    ErrorInternalServerError(HttpError::server_error(e.to_string()))
                })?;
            }
            Ok(response)
        }
        .boxed_local()
    }
}

/// Public middleware type used in route configuration:
/// `.wrap(RequireAuth)`
pub struct RequireAuth;

/// Factory that creates `AuthMiddleware` and wraps the inner service
impl<S> actix_web::dev::Transform<S, ServiceRequest> for RequireAuth
where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse<actix_web::body::BoxBody>,
            Error = actix_web::Error,
        > + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}
