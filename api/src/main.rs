mod config;
use crate::config::Config;
use crate::db::DbClient;
use crate::service::{auth_service::AuthService, user_service::UserService};
use crate::utils::email::EmailService;
use actix_web::{App, HttpServer, web};
use std::sync::Arc;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;
mod db;
mod dtos;
mod errors;
mod handler;
mod middleware;
mod models;
mod service;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pub db_client: DbClient,
    pub config: Config,
    pub auth_service: AuthService,
    pub user_service: UserService,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    init_logging();

    let config = Config::init();
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(config.database_url.as_str())
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    println!("Migrations executed successfully");

    let db_client = DbClient::new(pool);
    let email_service = EmailService::new(config.clone()).await;
    let app_state = AppState {
        config: config.clone(),
        db_client: db_client.clone(),
        auth_service: AuthService::new(
            Arc::new(db_client.auth.clone()),
            Arc::new(email_service.clone()),
            config.clone(),
        ),
        user_service: UserService::new(db_client.user.clone(), config.clone()),
    };

    println!("API starting on 0.0.0.0:{}", config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(handler::auth_handler::auth_handler())
            .service(handler::user_handler::user_handler())
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await?;

    Ok(())
}

pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();
}
