mod config;
use crate::config::Config;
use crate::db::DbClient;
use crate::service::reference_service::ReferenceService;
use crate::service::{auth_service::AuthService, user_service::UserService};
use crate::utils::email::EmailService;
use crate::utils::file_storage::FileStorageType;
use crate::utils::generic::MemoryCache;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use moka::future::Cache;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::time::Duration;
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
    pub reference_service: ReferenceService,
    pub embedding_model: Arc<TextEmbedding>,
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
    let embedding_model = Arc::new(
        TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
        )
        .expect("Failed to initialize embedding model"),
    );
    let cache = Cache::builder()
        .max_capacity(20)
        .time_to_live(Duration::from_secs((60 * 60) * 24)) //one day
        .build();
    let mem_cache = MemoryCache::new(cache);

    let app_state = AppState {
        config: config.clone(),
        db_client: db_client.clone(),
        auth_service: AuthService::new(
            Arc::new(db_client.auth.clone()),
            Arc::new(db_client.user.clone()),
            Arc::new(email_service.clone()),
            config.clone(),
        ),
        user_service: UserService::new(
            Arc::new(db_client.user.clone()),
            Arc::new(FileStorageType::UserImage),
        ),
        reference_service: ReferenceService::new(
            Arc::new(db_client.reference.clone()),
            mem_cache.clone(),
        ),
        embedding_model,
    };

    println!("API starting on 0.0.0.0:{}", config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(handler::auth_handler::auth_handler())
            .service(handler::user_handler::user_handler())
            .service(handler::reference_handler::reference_handler())
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
