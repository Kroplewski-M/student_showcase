use actix_web::{Responder, dev::HttpServiceFactory, web};

use crate::middleware::auth::RequireAuth;

pub fn user_handler() -> impl HttpServiceFactory {
    web::scope("/user")
        .wrap(RequireAuth)
        .route("/test", web::get().to(test))
}

pub async fn test() -> impl Responder {
    "test"
}
