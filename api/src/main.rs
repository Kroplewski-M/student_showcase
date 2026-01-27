use actix_web::cookie::Cookie;
use actix_web::cookie::time::Duration;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

use crate::db::DbClient;
mod db;
#[get("/health")]
async fn health() -> impl Responder {
    let is_prod = std::env::var("RUST_ENV").unwrap_or_default() == "production";
    let cookie = Cookie::build("health_test", "alive")
        .path("/")
        .http_only(true)
        .secure(is_prod) // enable in prod HTTPS
        .same_site(actix_web::cookie::SameSite::Lax)
        .max_age(Duration::days(1))
        .finish();

    HttpResponse::Ok().cookie(cookie).body("Ok")
}
#[get("/health/check")]
async fn health_check(req: HttpRequest) -> impl Responder {
    let has_cookie = req.cookie("health_test").is_some();
    if has_cookie {
        return "true";
    }
    "false"
}
#[derive(Debug, Clone)]
pub struct AppState {
    pub db_client: DbClient,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    unsafe {
        openssl_probe::try_init_openssl_env_vars();
    }
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(std::env::var("DATABASE_URL").unwrap().as_str())
        .await?;
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("Migrations executed successfully"),
        Err(e) => println!("Error running migrations: {}", e),
    }

    let db_client = DbClient::new(pool);
    let app_state = AppState { db_client };

    println!("API starting on 0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(health)
            .service(health_check)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
