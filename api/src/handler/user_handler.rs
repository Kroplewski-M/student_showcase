use actix_web::{Responder, Scope, web};

pub fn user_handler() -> Scope {
    web::scope("/user").route("/test", web::get().to(test))
}

pub async fn test() -> impl Responder {
    "test"
}
