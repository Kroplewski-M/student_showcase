use actix_web::{dev::HttpServiceFactory, web};

use crate::middleware::auth::RequireAuth;

pub fn admin_handler() -> impl HttpServiceFactory {
    web::scope("/admin").service(web::scope("").wrap(RequireAuth::admin()))
}
