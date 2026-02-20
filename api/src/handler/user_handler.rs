use actix_web::{Scope, web};

pub fn user_handler() -> Scope {
    web::scope("/user")
}
