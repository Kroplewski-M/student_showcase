use crate::errors::{ErrorMessage, ErrorResponse, HttpError};
use crate::{AppState, utils};
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
use std::rc::Rc;

/// Newtype wrapper around a user ID that has already been authenticated.
/// This is what handlers will extract once authentication succeeds.
#[derive(Clone)]
pub struct AuthenticatedUserId(pub String);

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
        let app_state = req.app_data::<web::Data<AppState>>().unwrap();
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

        let user_id = match utils::token::decode_token(
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
            let user_id = user_id.to_string();
            let exists = cloned_app_state
                .db_client
                .users
                .exists_verified(&user_id)
                .await
                .map_err(|e| ErrorInternalServerError(HttpError::server_error(e.to_string())))?;

            if exists {
                req.extensions_mut().insert(AuthenticatedUserId(user_id));
                srv.call(req).await
            } else {
                Err(ErrorForbidden(ErrorResponse {
                    status: "fail".into(),
                    message: ErrorMessage::PermissionDenied.to_string(),
                }))
            }
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
